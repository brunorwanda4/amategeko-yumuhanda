import java.io.File
import java.util.Properties

// App module build.gradle.kts for Amategeko y'Umuhanda.
//
// This module packages the pre-compiled Rust native library into an APK
// that uses Android's NativeActivity to host the GPUI application.
//
// The Rust library must be compiled separately and placed into
// app/src/main/jniLibs/<abi>/ before building the APK.

plugins {
    id("com.android.application")
}

val keystorePropertiesFile = rootProject.file("keystore.properties")
val keystoreProperties = Properties().apply {
    if (keystorePropertiesFile.isFile) {
        keystorePropertiesFile.inputStream().use(::load)
    }
}

fun signingValue(property: String, environment: String): String? =
    keystoreProperties.getProperty(property)?.takeIf(String::isNotBlank)
        ?: System.getenv(environment)?.takeIf(String::isNotBlank)

val configuredStoreFile = signingValue("storeFile", "ANDROID_KEYSTORE_PATH")?.let { path ->
    File(path).let { file ->
        if (file.isAbsolute) file else keystorePropertiesFile.parentFile.resolve(file)
    }
}
val configuredStorePassword = signingValue("storePassword", "ANDROID_KEYSTORE_PASSWORD")
val configuredKeyAlias = signingValue("keyAlias", "ANDROID_KEY_ALIAS")
val configuredKeyPassword = signingValue("keyPassword", "ANDROID_KEY_PASSWORD")
val releaseSigningConfigured = configuredStoreFile?.isFile == true &&
    configuredStorePassword != null && configuredKeyAlias != null && configuredKeyPassword != null
val allowDebugSigning = providers.gradleProperty("allowDebugSigning").orNull == "true"

gradle.taskGraph.whenReady {
    val buildsRelease = allTasks.any { task -> task.name.contains("Release", ignoreCase = true) }
    if (buildsRelease && !releaseSigningConfigured && !allowDebugSigning) {
        throw GradleException(
            "Release signing is not configured. Add android/keystore.properties or set the " +
                "ANDROID_KEYSTORE_* environment variables. For local testing only, run the " +
                "mobile build script with --allow-debug-signing."
        )
    }
}

android {
    namespace = "dev.gpui.mobile.example"
    compileSdk = 34

    defaultConfig {
        applicationId = "dev.gpui.mobile.example"
        minSdk = 26          // Vulkan 1.0 is mandatory from API 26+
        targetSdk = 34
        versionCode = 10800
        versionName = "1.8.0"

        // Tell NativeActivity which .so to load.
        // This must match the cdylib / example output name.
        ndk {
            abiFilters += listOf("arm64-v8a")
        }

        // Forward the library name to the manifest via a placeholder.
        manifestPlaceholders["nativeLibraryName"] = "gpui_mobile_app"
    }

    flavorDimensions += "distribution"
    productFlavors {
        create("sideload") {
            dimension = "distribution"
        }
        create("play") {
            dimension = "distribution"
        }
    }

    signingConfigs {
        getByName("debug") {
            enableV2Signing = true
            enableV3Signing = true
        }
        create("release") {
            if (releaseSigningConfigured) {
                storeFile = configuredStoreFile
                storePassword = configuredStorePassword
                keyAlias = configuredKeyAlias
                keyPassword = configuredKeyPassword
            }
            enableV2Signing = true
            enableV3Signing = true
        }
    }

    buildTypes {
        release {
            signingConfig = signingConfigs.getByName(
                if (allowDebugSigning) "debug" else "release"
            )
            isMinifyEnabled = true
            isShrinkResources = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
        debug {
            isDebuggable = true
            isJniDebuggable = true
        }
    }

    // We do NOT use CMake / ndk-build — the native library is compiled
    // externally via cargo-ndk and placed directly into jniLibs.
    externalNativeBuild {
        // Intentionally left empty.
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }

    // Tell Gradle where the pre-built .so files live.
    sourceSets {
        getByName("main") {
            jniLibs.srcDirs("src/main/jniLibs")
        }
    }

    // Lint configuration — relaxed for an example project.
    lint {
        abortOnError = false
        checkReleaseBuilds = false
    }
}

dependencies {
    // AndroidX back dispatcher used by MainActivity.
    implementation("androidx.activity:activity:1.9.3")
    // AndroidX core for NotificationCompat (used by GpuiNotifications)
    implementation("androidx.core:core:1.12.0")
    // AndroidX SplashScreen compat (used by GpuiActivity to hold splash until native init)
    implementation("androidx.core:core-splashscreen:1.0.1")
}
