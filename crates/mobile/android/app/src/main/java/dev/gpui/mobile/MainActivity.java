package dev.gpui.mobile;

import android.os.Build;
import android.os.Bundle;
import android.view.KeyEvent;

import androidx.activity.OnBackPressedCallback;
import androidx.activity.OnBackPressedDispatcher;
import androidx.activity.OnBackPressedDispatcherOwner;
import androidx.annotation.NonNull;
import androidx.lifecycle.Lifecycle;
import androidx.lifecycle.LifecycleOwner;
import androidx.lifecycle.LifecycleRegistry;

public class MainActivity extends GpuiActivity implements LifecycleOwner, OnBackPressedDispatcherOwner {

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
    }

    @Override
    protected void onPause() {
        mLifecycleRegistry.handleLifecycleEvent(Lifecycle.Event.ON_PAUSE);
        super.onPause();
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
