package dev.dioxus.main

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.os.Bundle
import android.os.Build
import app.skypia.messenger.BuildConfig

class MainActivity : WryActivity() {
    private var spotifyReceiver: BroadcastReceiver? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
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

    override fun onDestroy() {
        super.onDestroy()
        spotifyReceiver?.let {
            unregisterReceiver(it)
        }
    }
}
