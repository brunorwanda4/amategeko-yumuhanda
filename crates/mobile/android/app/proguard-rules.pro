# ProGuard / R8 rules for the GPUI Mobile Android Example.
#
# This is a pure native (Rust) application using NativeActivity — there is no
# Java or Kotlin application code to shrink, optimize, or obfuscate.
#
# These rules are intentionally empty.  They exist only because the
# build.gradle.kts references this file in the release buildType's
# proguardFiles configuration.

# These bridge methods are called by name from Rust/JNI. R8 cannot discover
# those calls and otherwise removes them from release builds.
-keep class dev.gpui.mobile.GpuiActivity { *; }
-keep class dev.gpui.mobile.MainActivity { *; }
