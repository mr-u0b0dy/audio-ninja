// Audio Ninja GUI - Frontend Application
const { invoke } = window.__TAURI__.tauri;

// Application state
let appState = {
    config: {
        sample_rate: 48000,
        drc_preset: 'Music',
        loudness_target: 'StreamingMusic',
        headroom_db: 3.0,
        headroom_lookahead_ms: 3.0,
        binaural_enabled: false,
        binaural_profile: 'ClosedBack',
        binaural_azimuth: 0.0,
        binaural_elevation: 0.0,
        binaural_distance: 1.0,
    }
};

// DRC preset info
const drcPresetInfo = {
    '': 'DRC disabled',
    'Speech': 'Speech: 3:1 ratio, -16dB threshold, 5ms/80ms attack/release (podcasts, audiobooks)',
    'Music': 'Music: 4:1 ratio, -18dB threshold, 10ms/100ms attack/release (streaming, mixed)',
    'Cinema': 'Cinema: 2:1 ratio, -14dB threshold, 20ms/150ms attack/release (film distribution)',
};

// Loudness target info
const loudnessInfo = {
    '': 'Loudness normalization disabled',
    'Television': 'Television: -23 LUFS (broadcast TV, streaming video)',
    'StreamingMusic': 'Streaming Music: -14 LUFS (Spotify, Apple Music, YouTube Music)',
    'FilmTheatrical': 'Film Theatrical: -27 LUFS (cinema distribution)',
    'FilmHome': 'Film Home: -20 LUFS (home video, Blu-ray, streaming films)',
};

// Initialize on page load
document.addEventListener('DOMContentLoaded', () => {
    setupEventListeners();
    initializeRenderer();
    updateUIFromState();
});

// Setup event listeners
function setupEventListeners() {
    // DRC
    document.getElementById('drcPreset').addEventListener('change', (e) => {
        appState.config.drc_preset = e.target.value;
        document.getElementById('drcInfo').textContent = drcPresetInfo[e.target.value] || '';
    });

    // Loudness
    document.getElementById('loudnessTarget').addEventListener('change', (e) => {
        appState.config.loudness_target = e.target.value;
        document.getElementById('loudnessInfo').textContent = loudnessInfo[e.target.value] || '';
    });

    // Headroom
    document.getElementById('headroomDb').addEventListener('input', (e) => {
        appState.config.headroom_db = parseFloat(e.target.value);
        document.getElementById('headroomDbValue').textContent = e.target.value;
    });

    document.getElementById('headroomLookahead').addEventListener('input', (e) => {
        appState.config.headroom_lookahead_ms = parseFloat(e.target.value);
        document.getElementById('headroomLookaheadValue').textContent = e.target.value;
    });

    // Binaural
    document.getElementById('binauralEnabled').addEventListener('change', (e) => {
        appState.config.binaural_enabled = e.target.checked;
        const controls = document.getElementById('binauralControls');
        if (e.target.checked) {
            controls.classList.remove('hidden');
        } else {
            controls.classList.add('hidden');
        }
    });

    document.getElementById('binauralProfile').addEventListener('change', (e) => {
        appState.config.binaural_profile = e.target.value;
    });

    document.getElementById('binauralAzimuth').addEventListener('input', (e) => {
        appState.config.binaural_azimuth = parseFloat(e.target.value);
        document.getElementById('binauralAzimuthValue').textContent = e.target.value + '°';
        updateSpatialViz();
    });

    document.getElementById('binauralElevation').addEventListener('input', (e) => {
        appState.config.binaural_elevation = parseFloat(e.target.value);
        document.getElementById('binauralElevationValue').textContent = e.target.value + '°';
        updateSpatialViz();
    });

    document.getElementById('binauralDistance').addEventListener('input', (e) => {
        appState.config.binaural_distance = parseFloat(e.target.value);
        document.getElementById('binauralDistanceValue').textContent = e.target.value.toFixed(1);
    });

    // Buttons
    document.getElementById('applyConfigBtn').addEventListener('click', applyConfiguration);
    document.getElementById('processAudioBtn').addEventListener('click', processAudio);
    document.getElementById('resetBtn').addEventListener('click', resetToDefaults);
}

// Initialize renderer
async function initializeRenderer() {
    try {
        const result = await invoke('initialize_renderer', { sampleRate: 48000 });
        showResult('Renderer initialized successfully', 'success');
    } catch (error) {
        showResult('Failed to initialize renderer: ' + error, 'error');
    }
}

// Apply configuration
async function applyConfiguration() {
    const btn = document.getElementById('applyConfigBtn');
    btn.disabled = true;
    btn.classList.add('loading');

    try {
        const config = {
            ...appState.config,
            drc_preset: appState.config.drc_preset || null,
            loudness_target: appState.config.loudness_target || null,
            binaural_profile: appState.config.binaural_profile || null,
        };

        const result = await invoke('apply_config', { config });

        // Update status display
        updateStatusDisplay(result);

        showResult(
            `✓ Configuration applied successfully\n` +
            `  DRC: ${result.drc_enabled ? 'Enabled' : 'Disabled'}\n` +
            `  Loudness: ${result.loudness_enabled ? 'Enabled' : 'Disabled'}\n` +
            `  Binaural: ${result.binaural_enabled ? 'Enabled' : 'Disabled'}`,
            'success'
        );
    } catch (error) {
        showResult('Failed to apply configuration: ' + error, 'error');
    } finally {
        btn.disabled = false;
        btn.classList.remove('loading');
    }
}

// Process audio
async function processAudio() {
    const btn = document.getElementById('processAudioBtn');
    btn.disabled = true;
    btn.classList.add('loading');

    try {
        const result = await invoke('process_audio', { channels: 2, numSamples: 48000 });

        showResult(
            `✓ Audio processed successfully\n` +
            `  Channels: ${result.channels}\n` +
            `  Samples: ${result.samples}\n` +
            `  Message: ${result.message}`,
            'success'
        );
    } catch (error) {
        showResult('Failed to process audio: ' + error, 'error');
    } finally {
        btn.disabled = false;
        btn.classList.remove('loading');
    }
}

// Reset to defaults
function resetToDefaults() {
    appState.config = {
        sample_rate: 48000,
        drc_preset: 'Music',
        loudness_target: 'StreamingMusic',
        headroom_db: 3.0,
        headroom_lookahead_ms: 3.0,
        binaural_enabled: false,
        binaural_profile: 'ClosedBack',
        binaural_azimuth: 0.0,
        binaural_elevation: 0.0,
        binaural_distance: 1.0,
    };
    updateUIFromState();
    showResult('Reset to default configuration', 'success');
}

// Update UI from state
function updateUIFromState() {
    document.getElementById('drcPreset').value = appState.config.drc_preset;
    document.getElementById('drcInfo').textContent = drcPresetInfo[appState.config.drc_preset];

    document.getElementById('loudnessTarget').value = appState.config.loudness_target;
    document.getElementById('loudnessInfo').textContent = loudnessInfo[appState.config.loudness_target];

    document.getElementById('headroomDb').value = appState.config.headroom_db;
    document.getElementById('headroomDbValue').textContent = appState.config.headroom_db.toFixed(1);

    document.getElementById('headroomLookahead').value = appState.config.headroom_lookahead_ms;
    document.getElementById('headroomLookaheadValue').textContent = appState.config.headroom_lookahead_ms.toFixed(1);

    document.getElementById('binauralEnabled').checked = appState.config.binaural_enabled;
    const controls = document.getElementById('binauralControls');
    if (appState.config.binaural_enabled) {
        controls.classList.remove('hidden');
    } else {
        controls.classList.add('hidden');
    }

    document.getElementById('binauralProfile').value = appState.config.binaural_profile;
    document.getElementById('binauralAzimuth').value = appState.config.binaural_azimuth;
    document.getElementById('binauralAzimuthValue').textContent = appState.config.binaural_azimuth + '°';

    document.getElementById('binauralElevation').value = appState.config.binaural_elevation;
    document.getElementById('binauralElevationValue').textContent = appState.config.binaural_elevation + '°';

    document.getElementById('binauralDistance').value = appState.config.binaural_distance;
    document.getElementById('binauralDistanceValue').textContent = appState.config.binaural_distance.toFixed(1);

    updateSpatialViz();
}

// Update status display
function updateStatusDisplay(status) {
    document.getElementById('statusSampleRate').textContent = status.sample_rate + ' Hz';
    
    const drcStatus = document.getElementById('statusDRC');
    if (status.drc_enabled) {
        drcStatus.textContent = 'Active';
        drcStatus.classList.remove('status-off');
        drcStatus.classList.add('status-on');
    } else {
        drcStatus.textContent = 'Inactive';
        drcStatus.classList.remove('status-on');
        drcStatus.classList.add('status-off');
    }

    const loudnessStatus = document.getElementById('statusLoudness');
    if (status.loudness_enabled) {
        loudnessStatus.textContent = 'Active';
        loudnessStatus.classList.remove('status-off');
        loudnessStatus.classList.add('status-on');
    } else {
        loudnessStatus.textContent = 'Inactive';
        loudnessStatus.classList.remove('status-on');
        loudnessStatus.classList.add('status-off');
    }

    const binauralStatus = document.getElementById('statusBinaural');
    if (status.binaural_enabled) {
        binauralStatus.textContent = 'Active';
        binauralStatus.classList.remove('status-off');
        binauralStatus.classList.add('status-on');
    } else {
        binauralStatus.textContent = 'Inactive';
        binauralStatus.classList.remove('status-on');
        binauralStatus.classList.add('status-off');
    }
}

// Update spatial visualization
function updateSpatialViz() {
    const azimuth = appState.config.binaural_azimuth;
    const elevation = appState.config.binaural_elevation;

    // Convert spherical to SVG coordinates
    // Center at 100,100 with radius 50
    const radius = 50;
    const angle = (azimuth + 90) * Math.PI / 180; // +90 to make 0° point up
    const elevation_factor = (90 - elevation) / 180; // normalize to 0-1

    const x = 100 + radius * elevation_factor * Math.cos(angle);
    const y = 100 - radius * elevation_factor * Math.sin(angle);

    const soundSource = document.getElementById('soundSource');
    soundSource.setAttribute('cx', x);
    soundSource.setAttribute('cy', y);
}

// Show results
function showResult(message, type = 'success') {
    const resultsContent = document.getElementById('resultsContent');
    resultsContent.textContent = message;
    resultsContent.className = 'results-content ' + type;
}

// Get available options (for future use)
async function getAvailableOptions() {
    try {
        const options = await invoke('get_available_options');
        console.log('Available options:', options);
        return options;
    } catch (error) {
        console.error('Failed to get options:', error);
    }
}
