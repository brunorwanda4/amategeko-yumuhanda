package dev.gpui.mobile;

import android.content.Intent;
import android.net.Uri;
import android.os.Build;
import android.os.Bundle;
import android.provider.Settings;
import android.view.KeyEvent;
import android.view.WindowManager;

import androidx.activity.OnBackPressedCallback;
import androidx.activity.OnBackPressedDispatcher;
import androidx.activity.OnBackPressedDispatcherOwner;
import androidx.annotation.NonNull;
import androidx.core.content.FileProvider;
import androidx.lifecycle.Lifecycle;
import androidx.lifecycle.LifecycleOwner;
import androidx.lifecycle.LifecycleRegistry;

import java.io.File;
import java.io.IOException;

public class MainActivity extends GpuiActivity implements LifecycleOwner, OnBackPressedDispatcherOwner {

    private boolean keepScreenOnRequested;
    private final LifecycleRegistry mLifecycleRegistry = new LifecycleRegistry(this);
    private final OnBackPressedDispatcher mBackPressedDispatcher = new OnBackPressedDispatcher(new Runnable() {
        @Override
        public void run() {
            MainActivity.super.onBackPressed();
        }
    });

    @NonNull
    @Override
    public Lifecycle getLifecycle() {
        return mLifecycleRegistry;
    }

    @NonNull
    @Override
    public OnBackPressedDispatcher getOnBackPressedDispatcher() {
        return mBackPressedDispatcher;
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        mLifecycleRegistry.handleLifecycleEvent(Lifecycle.Event.ON_CREATE);

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            mBackPressedDispatcher.setOnBackInvokedDispatcher(getOnBackInvokedDispatcher());
        }

        mBackPressedDispatcher.addCallback(this, new OnBackPressedCallback(true) {
            @Override
            public void handleOnBackPressed() {
                if (!nativeOnBack()) {
                    setEnabled(false);
                    mBackPressedDispatcher.onBackPressed();
                }
            }
        });

        super.onCreate(savedInstanceState);
    }

    @Override
    protected void onStart() {
        super.onStart();
        mLifecycleRegistry.handleLifecycleEvent(Lifecycle.Event.ON_START);
    }

    @Override
    protected void onResume() {
        super.onResume();
        mLifecycleRegistry.handleLifecycleEvent(Lifecycle.Event.ON_RESUME);
        updateKeepScreenOn();
    }

    @Override
    protected void onPause() {
        getWindow().clearFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON);
        mLifecycleRegistry.handleLifecycleEvent(Lifecycle.Event.ON_PAUSE);
        super.onPause();
    }

    public void setKeepScreenOn(boolean on) {
        keepScreenOnRequested = on;
        runOnUiThread(this::updateKeepScreenOn);
    }

    private void updateKeepScreenOn() {
        if (keepScreenOnRequested) {
            getWindow().addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON);
        } else {
            getWindow().clearFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON);
        }
    }

    public int requestInstallUpdate(String apkPath) {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O
                && !getPackageManager().canRequestPackageInstalls()) {
            runOnUiThread(() -> {
                Intent intent = new Intent(
                        Settings.ACTION_MANAGE_UNKNOWN_APP_SOURCES,
                        Uri.parse("package:" + getPackageName()));
                startActivity(intent);
            });
            return 1;
        }

        try {
            File apk = new File(apkPath).getCanonicalFile();
            File updateDir = new File(getCacheDir(), "updates").getCanonicalFile();
            if (!apk.isFile() || !apk.getPath().startsWith(updateDir.getPath() + File.separator)) {
                return -1;
            }
            Uri uri = FileProvider.getUriForFile(
                    this, getPackageName() + ".fileprovider", apk);
            runOnUiThread(() -> {
                Intent intent = new Intent(Intent.ACTION_VIEW);
                intent.setDataAndType(uri, "application/vnd.android.package-archive");
                intent.addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION);
                startActivity(intent);
            });
            return 0;
        } catch (IOException | RuntimeException error) {
            return -1;
        }
    }

    public String updateCachePath() {
        return new File(getCacheDir(), "updates").getAbsolutePath();
    }

    public void openExternalUrl(String url) {
        runOnUiThread(() -> startActivity(new Intent(Intent.ACTION_VIEW, Uri.parse(url))));
    }

    @Override
    protected void onStop() {
        mLifecycleRegistry.handleLifecycleEvent(Lifecycle.Event.ON_STOP);
        super.onStop();
    }

    @Override
    protected void onDestroy() {
        mLifecycleRegistry.handleLifecycleEvent(Lifecycle.Event.ON_DESTROY);
        super.onDestroy();
    }

    @Override
    public void onBackPressed() {
        if (mBackPressedDispatcher.hasEnabledCallbacks()) {
            mBackPressedDispatcher.onBackPressed();
        } else {
            super.onBackPressed();
        }
    }

    @Override
    public boolean dispatchKeyEvent(KeyEvent event) {
        if (event.getKeyCode() == KeyEvent.KEYCODE_BACK) {
            if (event.getAction() == KeyEvent.ACTION_UP) {
                mBackPressedDispatcher.onBackPressed();
            }
            return true;
        }
        return super.dispatchKeyEvent(event);
    }

    public native boolean nativeOnBack();
}
