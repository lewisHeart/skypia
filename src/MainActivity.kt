package dev.dioxus.main

import android.Manifest
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.content.pm.PackageManager
import android.os.Bundle
import android.os.Build
import android.view.View
import android.view.ViewGroup
import android.webkit.PermissionRequest
import android.webkit.WebChromeClient
import android.webkit.WebView
import app.skypia.messenger.BuildConfig

class MainActivity : WryActivity() {
    private var spotifyReceiver: BroadcastReceiver? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Solicita permissão do sistema operacional Android para gravação de áudio
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
            if (checkSelfPermission(Manifest.permission.RECORD_AUDIO) != PackageManager.PERMISSION_GRANTED) {
                requestPermissions(arrayOf(Manifest.permission.RECORD_AUDIO), 200)
            }
        }

        // Busca a WebView na árvore de views e configura o WebChromeClient de forma assíncrona
        setupWebViewDelayed(15)
        
        val filter = IntentFilter()
        filter.addAction("com.spotify.music.metachanged")
        filter.addAction("com.spotify.music.playbackstatechanged")
        filter.addAction("com.spotify.music.queuechanged")
        
        spotifyReceiver = object : BroadcastReceiver() {
            override fun onReceive(context: Context, intent: Intent) {
                val action = intent.action ?: return
                val sharedPref = context.getSharedPreferences("spotify_pref", Context.MODE_PRIVATE)
                val editor = sharedPref.edit()
                
                if (action == "com.spotify.music.metachanged") {
                    val artist = intent.getStringExtra("artist") ?: ""
                    val track = intent.getStringExtra("track") ?: ""
                    val isPlaying = intent.getBooleanExtra("playing", false)
                    
                    if (artist.isNotEmpty() && track.isNotEmpty()) {
                        editor.putString("current_song", "$artist - $track")
                    }
                    editor.putBoolean("is_playing", isPlaying)
                } else if (action == "com.spotify.music.playbackstatechanged") {
                    val isPlaying = intent.getBooleanExtra("playing", false)
                    editor.putBoolean("is_playing", isPlaying)
                }
                editor.apply()
            }
        }
        
        if (Build.VERSION.SDK_INT >= 33) {
            registerReceiver(spotifyReceiver, filter, Context.RECEIVER_EXPORTED)
        } else {
            registerReceiver(spotifyReceiver, filter)
        }
    }

    private fun setupWebViewDelayed(attemptsLeft: Int) {
        if (attemptsLeft <= 0) return
        window.decorView.postDelayed({
            val webView = findWebView(window.decorView)
            if (webView != null) {
                setupWebChromeClient(webView)
            } else {
                setupWebViewDelayed(attemptsLeft - 1)
            }
        }, 400)
    }

    private fun findWebView(view: View): WebView? {
        if (view is WebView) {
            return view
        }
        if (view is ViewGroup) {
            for (i in 0 until view.childCount) {
                val child = view.getChildAt(i)
                val result = findWebView(child)
                if (result != null) {
                    return result
                }
            }
        }
        return null
    }

    private fun setupWebChromeClient(webView: WebView) {
        val originalClient = webView.webChromeClient
        webView.webChromeClient = object : WebChromeClient() {
            override fun onPermissionRequest(request: PermissionRequest?) {
                if (request != null) {
                    val resources = request.resources
                    for (resource in resources) {
                        if (resource == PermissionRequest.RESOURCE_AUDIO_CAPTURE) {
                            request.grant(arrayOf(resource))
                            return
                        }
                    }
                    request.grant(resources)
                }
            }

            override fun onConsoleMessage(consoleMessage: android.webkit.ConsoleMessage?): Boolean {
                return originalClient?.onConsoleMessage(consoleMessage) ?: super.onConsoleMessage(consoleMessage)
            }

            override fun onShowFileChooser(
                webView: WebView?,
                filePathCallback: android.webkit.ValueCallback<Array<android.net.Uri>>?,
                fileChooserParams: FileChooserParams?
            ): Boolean {
                return originalClient?.onShowFileChooser(webView, filePathCallback, fileChooserParams)
                    ?: super.onShowFileChooser(webView, filePathCallback, fileChooserParams)
            }
        }
    }

    override fun onDestroy() {
        super.onDestroy()
        spotifyReceiver?.let {
            unregisterReceiver(it)
        }
    }
}
