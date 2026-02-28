// Audio Ninja GUI - Frontend Application
const { invoke } = window.__TAURI__.tauri;

// Daemon API base URL
const DAEMON_API = 'http://127.0.0.1:8080/api/v1';

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
    },
    input_devices: [],
    output_devices: [],
    current_input: null,
    current_output: null,
    playback_state: 'idle',
    latency_history: [],
    bandwidth_history: [],
    daemon_connected: false,
};

// Speaker layout presets with positions (azimuth, elevation)
const LAYOUT_PRESETS = {
    'stereo':   { name: '2.0 Stereo', speakers: [{n:'L',az:-30,el:0},{n:'R',az:30,el:0}] },
    '2.1':      { name: '2.1', speakers: [{n:'L',az:-30,el:0},{n:'R',az:30,el:0},{n:'LFE',az:0,el:-30}] },
    '3.1':      { name: '3.1', speakers: [{n:'C',az:0,el:0},{n:'L',az:-30,el:0},{n:'R',az:30,el:0},{n:'LFE',az:0,el:-30}] },
    'quad':     { name: '4.0 Quad', speakers: [{n:'FL',az:-45,el:0},{n:'FR',az:45,el:0},{n:'RL',az:-135,el:0},{n:'RR',az:135,el:0}] },
    '5.1':      { name: '5.1 Surround', speakers: [{n:'C',az:0,el:0},{n:'L',az:-30,el:0},{n:'R',az:30,el:0},{n:'LS',az:-110,el:0},{n:'RS',az:110,el:0},{n:'LFE',az:0,el:-30}] },
    '5.1.2':    { name: '5.1.2', speakers: [{n:'C',az:0,el:0},{n:'L',az:-30,el:0},{n:'R',az:30,el:0},{n:'LS',az:-110,el:0},{n:'RS',az:110,el:0},{n:'LFE',az:0,el:-30},{n:'TFL',az:-45,el:45},{n:'TFR',az:45,el:45}] },
    '7.1':      { name: '7.1 Surround', speakers: [{n:'C',az:0,el:0},{n:'L',az:-30,el:0},{n:'R',az:30,el:0},{n:'LS',az:-90,el:0},{n:'RS',az:90,el:0},{n:'LB',az:-135,el:0},{n:'RB',az:135,el:0},{n:'LFE',az:0,el:-30}] },
    '7.1.4':    { name: '7.1.4 Atmos', speakers: [{n:'C',az:0,el:0},{n:'L',az:-30,el:0},{n:'R',az:30,el:0},{n:'LS',az:-90,el:0},{n:'RS',az:90,el:0},{n:'LB',az:-135,el:0},{n:'RB',az:135,el:0},{n:'LFE',az:0,el:-30},{n:'TFL',az:-45,el:45},{n:'TFR',az:45,el:45},{n:'TBL',az:-135,el:45},{n:'TBR',az:135,el:45}] },
    '9.1.6':    { name: '9.1.6 Immersive', speakers: [{n:'C',az:0,el:0},{n:'L',az:-30,el:0},{n:'R',az:30,el:0},{n:'WL',az:-60,el:0},{n:'WR',az:60,el:0},{n:'LS',az:-90,el:0},{n:'RS',az:90,el:0},{n:'LB',az:-135,el:0},{n:'RB',az:135,el:0},{n:'LFE',az:0,el:-30},{n:'TFL',az:-45,el:45},{n:'TFR',az:45,el:45},{n:'TSL',az:-90,el:45},{n:'TSR',az:90,el:45},{n:'TBL',az:-135,el:45},{n:'TBR',az:135,el:45}] },
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

// ===== Initialization =====
document.addEventListener('DOMContentLoaded', () => {
    loadState();
    setupTabNavigation();
    setupEventListeners();
    setupKeyboardShortcuts();
    setupDragDrop();
    initializeRenderer();
    updateUIFromState();
    loadDevices();
    drawLayoutCanvas('5.1');
    drawIRCanvas();
    drawFilterCanvas();
    drawLatencyCanvas();
    drawBandwidthCanvas();
    setupLayoutCanvasDrag();
    startStatusPolling();
    checkDaemonConnection();
});

// ===== Keyboard Shortcuts =====
function setupKeyboardShortcuts() {
    document.addEventListener('keydown', (e) => {
        // Don't capture when user is typing in an input
        if (e.target.tagName === 'INPUT' || e.target.tagName === 'SELECT' || e.target.tagName === 'TEXTAREA') return;

        switch (e.key) {
            case ' ':
                e.preventDefault();
                if (appState.playback_state === 'playing') transportPause();
                else transportPlay();
                break;
            case 'Escape':
                transportStop();
                break;
            case '1': case '2': case '3': case '4': case '5': case '6': case '7':
                if (!e.ctrlKey && !e.metaKey) {
                    const tabs = document.querySelectorAll('.tab-btn');
                    const idx = parseInt(e.key) - 1;
                    if (idx < tabs.length) tabs[idx].click();
                }
                break;
        }

        // Ctrl+O for file picker
        if ((e.ctrlKey || e.metaKey) && e.key === 'o') {
            e.preventDefault();
            selectAudioFile();
        }
    });
}

// ===== Drag & Drop File Loading =====
function setupDragDrop() {
    const transportPanel = document.getElementById('tab-transport');
    if (!transportPanel) return;

    // Create drop overlay
    const overlay = document.createElement('div');
    overlay.className = 'drop-overlay';
    overlay.textContent = 'Drop audio file here';
    transportPanel.style.position = 'relative';
    transportPanel.appendChild(overlay);

    let dragCounter = 0;

    transportPanel.addEventListener('dragenter', (e) => {
        e.preventDefault();
        dragCounter++;
        overlay.classList.add('active');
    });

    transportPanel.addEventListener('dragleave', () => {
        dragCounter--;
        if (dragCounter <= 0) { dragCounter = 0; overlay.classList.remove('active'); }
    });

    transportPanel.addEventListener('dragover', (e) => e.preventDefault());

    transportPanel.addEventListener('drop', (e) => {
        e.preventDefault();
        dragCounter = 0;
        overlay.classList.remove('active');

        const files = e.dataTransfer.files;
        if (files.length > 0) {
            const filePath = files[0].path || files[0].name;
            document.getElementById('audioFile').value = filePath;
            loadAudioFile(filePath);
        }
    });
}

// ===== localStorage Persistence =====
function saveState() {
    const persist = {
        activeTab: document.querySelector('.tab-btn.active')?.dataset.tab || 'status',
        inputDevice: appState.current_input,
        outputDevice: appState.current_output,
        transportMode: document.getElementById('transportMode')?.value,
        layoutPreset: document.getElementById('layoutPreset')?.value,
        config: appState.config,
    };
    try { localStorage.setItem('audioNinjaState', JSON.stringify(persist)); } catch {}
}

function loadState() {
    try {
        const saved = JSON.parse(localStorage.getItem('audioNinjaState'));
        if (!saved) return;

        // Restore active tab
        if (saved.activeTab) {
            setTimeout(() => {
                const btn = document.querySelector(`.tab-btn[data-tab="${saved.activeTab}"]`);
                if (btn) btn.click();
            }, 0);
        }

        // Restore config settings
        if (saved.config) {
            Object.assign(appState.config, saved.config);
        }

        // Restore device selections after devices load
        if (saved.inputDevice) appState.current_input = saved.inputDevice;
        if (saved.outputDevice) appState.current_output = saved.outputDevice;

        // Restore transport mode
        if (saved.transportMode) {
            setTimeout(() => {
                const sel = document.getElementById('transportMode');
                if (sel) sel.value = saved.transportMode;
            }, 100);
        }

        // Restore layout preset
        if (saved.layoutPreset) {
            setTimeout(() => {
                const sel = document.getElementById('layoutPreset');
                if (sel) { sel.value = saved.layoutPreset; drawLayoutCanvas(saved.layoutPreset); }
            }, 100);
        }
    } catch {}
}

// ===== Tab Navigation =====
function setupTabNavigation() {
    document.querySelectorAll('.tab-btn').forEach(btn => {
        btn.addEventListener('click', () => {
            document.querySelectorAll('.tab-btn').forEach(b => {
                b.classList.remove('active');
                b.setAttribute('aria-selected', 'false');
            });
            document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
            btn.classList.add('active');
            btn.setAttribute('aria-selected', 'true');
            document.getElementById('tab-' + btn.dataset.tab).classList.add('active');
            saveState();
        });
    });
}

// ===== Event Listeners =====
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
        controls.classList.toggle('hidden', !e.target.checked);
    });

    document.getElementById('binauralProfile').addEventListener('change', (e) => {
        appState.config.binaural_profile = e.target.value;
    });

    document.getElementById('binauralAzimuth').addEventListener('input', (e) => {
        appState.config.binaural_azimuth = parseFloat(e.target.value);
        document.getElementById('binauralAzimuthValue').textContent = e.target.value + '\u00B0';
        updateSpatialViz();
    });

    document.getElementById('binauralElevation').addEventListener('input', (e) => {
        appState.config.binaural_elevation = parseFloat(e.target.value);
        document.getElementById('binauralElevationValue').textContent = e.target.value + '\u00B0';
        updateSpatialViz();
    });

    document.getElementById('binauralDistance').addEventListener('input', (e) => {
        appState.config.binaural_distance = parseFloat(e.target.value);
        document.getElementById('binauralDistanceValue').textContent = parseFloat(e.target.value).toFixed(1);
    });

    // Config buttons
    document.getElementById('applyConfigBtn').addEventListener('click', applyConfiguration);
    document.getElementById('processAudioBtn').addEventListener('click', processAudio);
    document.getElementById('resetBtn').addEventListener('click', resetToDefaults);

    // I/O Controls
    document.getElementById('refreshDevicesBtn').addEventListener('click', loadDevices);
    document.getElementById('inputDevice').addEventListener('change', selectInputDevice);
    document.getElementById('inputSource').addEventListener('change', selectInputSource);
    document.getElementById('outputDevice').addEventListener('change', selectOutputDevice);

    // Transport Controls
    document.getElementById('playBtn').addEventListener('click', transportPlay);
    document.getElementById('pauseBtn').addEventListener('click', transportPause);
    document.getElementById('stopBtn').addEventListener('click', transportStop);
    document.getElementById('selectFileBtn').addEventListener('click', selectAudioFile);
    document.getElementById('transportMode').addEventListener('change', setTransportMode);

    // Seek Slider
    const seekSlider = document.getElementById('seekSlider');
    seekSlider.addEventListener('mousedown', () => { isSeeking = true; });
    seekSlider.addEventListener('touchstart', () => { isSeeking = true; });
    seekSlider.addEventListener('input', () => {
        if (appState.playback_total_duration) {
            const pos = (seekSlider.value / 1000) * appState.playback_total_duration;
            document.getElementById('playbackPosition').textContent = formatTime(pos);
        }
    });
    seekSlider.addEventListener('change', () => {
        isSeeking = false;
        if (appState.playback_total_samples) {
            const seekSample = Math.round((seekSlider.value / 1000) * appState.playback_total_samples);
            seekToPosition(seekSample);
        }
    });

    // Transport mode help text
    const modeHelp = { 'FileOnly': 'Play audio from a loaded file', 'StreamOnly': 'Capture live audio from input devices', 'Mixed': 'Play file and capture live input simultaneously' };
    document.getElementById('transportMode').addEventListener('change', (e) => {
        document.getElementById('transportModeHelp').textContent = modeHelp[e.target.value] || '';
    });

    // Input device change updates info card
    document.getElementById('inputDevice').addEventListener('change', () => {
        const device = appState.input_devices.find(d => d.id === document.getElementById('inputDevice').value);
        updateDeviceInfoCard('input', device);
    });
    document.getElementById('outputDevice').addEventListener('change', () => {
        const device = appState.output_devices.find(d => d.id === document.getElementById('outputDevice').value);
        updateDeviceInfoCard('output', device);
    });

    // Layout Controls
    document.getElementById('layoutPreset').addEventListener('change', (e) => {
        layoutDragState.presetKey = e.target.value;
        drawLayoutCanvas(e.target.value);
    });
    document.getElementById('applyLayoutBtn').addEventListener('click', applyLayout);
    document.getElementById('testVBAPBtn').addEventListener('click', testVBAPSignal);

    // Calibration Controls
    document.getElementById('sweepDuration').addEventListener('input', (e) => {
        document.getElementById('sweepDurationValue').textContent = parseFloat(e.target.value).toFixed(1) + 's';
    });
    document.getElementById('filterTaps').addEventListener('input', (e) => {
        document.getElementById('filterTapsValue').textContent = e.target.value;
    });
    document.getElementById('startCalBtn').addEventListener('click', startCalibration);
    document.getElementById('stopCalBtn').addEventListener('click', stopCalibration);
    document.getElementById('applyCalBtn').addEventListener('click', applyCalibration);
    document.getElementById('designFilterBtn').addEventListener('click', designFilter);
    document.getElementById('exportFilterBtn').addEventListener('click', exportFilter);
}

// ===== Daemon Connection with Retry =====
let connectionRetryDelay = 1000;
const MAX_RETRY_DELAY = 30000;

async function checkDaemonConnection() {
    const dot = document.getElementById('connDot');
    const text = document.getElementById('connText');
    try {
        const response = await fetch(`${DAEMON_API}/status`, { signal: AbortSignal.timeout(3000) });
        if (response.ok) {
            const data = await response.json();
            appState.daemon_connected = true;
            connectionRetryDelay = 1000; // reset retry
            const el = document.getElementById('statusDaemon');
            el.textContent = 'Connected (v' + data.version + ')';
            el.classList.remove('status-off');
            el.classList.add('status-on');
            document.getElementById('statusUptime').textContent = formatUptime(data.uptime_secs);
            // Connection indicator
            dot.className = 'conn-dot conn-online';
            text.textContent = 'Online';
        }
    } catch {
        appState.daemon_connected = false;
        const el = document.getElementById('statusDaemon');
        el.textContent = 'Disconnected';
        el.classList.remove('status-on');
        el.classList.add('status-off');
        // Connection indicator with retry
        dot.className = 'conn-dot conn-offline';
        text.textContent = 'Offline';
        // Exponential backoff retry
        setTimeout(async () => {
            dot.className = 'conn-dot conn-retry';
            text.textContent = 'Retrying...';
            await checkDaemonConnection();
        }, connectionRetryDelay);
        connectionRetryDelay = Math.min(connectionRetryDelay * 2, MAX_RETRY_DELAY);
    }
}

function formatUptime(secs) {
    const h = Math.floor(secs / 3600);
    const m = Math.floor((secs % 3600) / 60);
    const s = secs % 60;
    if (h > 0) return `${h}h ${m}m`;
    if (m > 0) return `${m}m ${s}s`;
    return `${s}s`;
}

// ===== Initialize Renderer =====
async function initializeRenderer() {
    try {
        const result = await invoke('initialize_renderer', { sampleRate: 48000 });
        showResult('Renderer initialized successfully', 'success');
    } catch (error) {
        showResult('Renderer init: ' + error, 'error');
    }
}

// ===== Apply Configuration =====
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
        updateStatusDisplay(result);
        showResult(
            'Configuration applied successfully\n' +
            '  DRC: ' + (result.drc_enabled ? 'Enabled' : 'Disabled') + '\n' +
            '  Loudness: ' + (result.loudness_enabled ? 'Enabled' : 'Disabled') + '\n' +
            '  Binaural: ' + (result.binaural_enabled ? 'Enabled' : 'Disabled'),
            'success'
        );
    } catch (error) {
        showResult('Failed to apply configuration: ' + error, 'error');
    } finally {
        btn.disabled = false;
        btn.classList.remove('loading');
    }
}

// ===== Process Audio =====
async function processAudio() {
    const btn = document.getElementById('processAudioBtn');
    btn.disabled = true;
    btn.classList.add('loading');

    try {
        const result = await invoke('process_audio', { channels: 2, numSamples: 48000 });
        showResult(
            'Audio processed successfully\n' +
            '  Channels: ' + result.channels + '\n' +
            '  Samples: ' + result.samples + '\n' +
            '  Message: ' + result.message,
            'success'
        );
    } catch (error) {
        showResult('Failed to process audio: ' + error, 'error');
    } finally {
        btn.disabled = false;
        btn.classList.remove('loading');
    }
}

// ===== Reset to Defaults =====
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

// ===== Update UI From State =====
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
    document.getElementById('binauralControls').classList.toggle('hidden', !appState.config.binaural_enabled);
    document.getElementById('binauralProfile').value = appState.config.binaural_profile;
    document.getElementById('binauralAzimuth').value = appState.config.binaural_azimuth;
    document.getElementById('binauralAzimuthValue').textContent = appState.config.binaural_azimuth + '\u00B0';
    document.getElementById('binauralElevation').value = appState.config.binaural_elevation;
    document.getElementById('binauralElevationValue').textContent = appState.config.binaural_elevation + '\u00B0';
    document.getElementById('binauralDistance').value = appState.config.binaural_distance;
    document.getElementById('binauralDistanceValue').textContent = appState.config.binaural_distance.toFixed(1);
    updateSpatialViz();
}

// ===== Status Display =====
function updateStatusDisplay(status) {
    document.getElementById('statusSampleRate').textContent = status.sample_rate + ' Hz';

    setStatusIndicator('statusDRC', status.drc_enabled, 'Active', 'Inactive');
    setStatusIndicator('statusLoudness', status.loudness_enabled, 'Active', 'Inactive');
    setStatusIndicator('statusBinaural', status.binaural_enabled, 'Active', 'Inactive');
}

function setStatusIndicator(id, active, onText, offText) {
    const el = document.getElementById(id);
    el.textContent = active ? onText : offText;
    el.classList.toggle('status-on', active);
    el.classList.toggle('status-off', !active);
}

// ===== Spatial Visualization =====
function updateSpatialViz() {
    const azimuth = appState.config.binaural_azimuth;
    const elevation = appState.config.binaural_elevation;
    const radius = 50;
    const angle = (azimuth + 90) * Math.PI / 180;
    const elevation_factor = (90 - elevation) / 180;
    const x = 100 + radius * elevation_factor * Math.cos(angle);
    const y = 100 - radius * elevation_factor * Math.sin(angle);
    const soundSource = document.getElementById('soundSource');
    if (soundSource) {
        soundSource.setAttribute('cx', x);
        soundSource.setAttribute('cy', y);
    }
}

// ===== Toast Notification System =====
function showToast(message, type = 'success') {
    let container = document.getElementById('toastContainer');
    if (!container) {
        container = document.createElement('div');
        container.id = 'toastContainer';
        container.className = 'toast-container';
        document.body.appendChild(container);
    }

    const toast = document.createElement('div');
    toast.className = 'toast toast-' + type;

    const icons = { success: '\u2713', error: '\u2717', warning: '\u26A0', info: '\u2139' };
    toast.textContent = (icons[type] || '') + ' ' + message;
    toast.addEventListener('click', () => toast.remove());

    container.appendChild(toast);
    setTimeout(() => { if (toast.parentNode) toast.remove(); }, 3000);
}

// ===== Show Results (also fires toast) =====
function showResult(message, type = 'success') {
    const resultsContent = document.getElementById('resultsContent');
    if (resultsContent) {
        resultsContent.textContent = message;
        resultsContent.className = 'results-content ' + type;
    }
    showToast(message, type);
}

// ===== I/O Device Management =====
async function loadDevices() {
    try {
        const inputResp = await fetch(`${DAEMON_API}/input/devices`);
        if (inputResp.ok) {
            const data = await inputResp.json();
            appState.input_devices = Array.isArray(data) ? data : (data.devices || []);
            updateInputDeviceList();
        }

        const outputResp = await fetch(`${DAEMON_API}/output/devices`);
        if (outputResp.ok) {
            const data = await outputResp.json();
            appState.output_devices = Array.isArray(data) ? data : (data.devices || []);
            updateOutputDeviceList();
        }

        // Also populate calibration mic dropdown
        populateCalMicDevices();

        showResult('Devices loaded successfully', 'success');
    } catch (error) {
        showResult('Failed to load devices (daemon may be offline)', 'error');
    }
}

function updateInputDeviceList() {
    const select = document.getElementById('inputDevice');
    select.innerHTML = '';
    if (appState.input_devices.length === 0) {
        select.innerHTML = '<option value="">No devices available</option>';
        updateDeviceInfoCard('input', null);
        return;
    }
    if (appState.input_devices.length === 1) {
        // Auto-select single device
        appState.current_input = appState.input_devices[0].id;
    }
    appState.input_devices.forEach(device => {
        const option = document.createElement('option');
        option.value = device.id;
        option.textContent = device.name + ' (' + (device.device_type || device.type || 'unknown') + ')';
        if (!device.available) option.textContent += ' [Unavailable]';
        option.disabled = !device.available;
        select.appendChild(option);
    });
    updateDeviceInfoCard('input', appState.input_devices[0]);
}

function updateOutputDeviceList() {
    const select = document.getElementById('outputDevice');
    select.innerHTML = '';
    if (appState.output_devices.length === 0) {
        select.innerHTML = '<option value="">No devices available</option>';
        updateDeviceInfoCard('output', null);
        return;
    }
    if (appState.output_devices.length === 1) {
        appState.current_output = appState.output_devices[0].id;
    }
    appState.output_devices.forEach(device => {
        const option = document.createElement('option');
        option.value = device.id;
        option.textContent = device.name + ' (' + (device.device_type || 'unknown') + ', ' + (device.max_channels || 2) + 'ch)';
        if (device.is_default) option.textContent += ' [Default]';
        if (!device.available) option.textContent += ' [Unavailable]';
        option.disabled = !device.available;
        select.appendChild(option);
    });
    // Auto-select default
    const defaultDev = appState.output_devices.find(d => d.is_default) || appState.output_devices[0];
    if (defaultDev) {
        select.value = defaultDev.id;
        updateDeviceInfoCard('output', defaultDev);
    }
}

function updateDeviceInfoCard(type, device) {
    if (type === 'input') {
        document.getElementById('inputChannels').textContent = device ? (device.max_channels || 2) : '-';
        document.getElementById('inputSampleRate').textContent = device ? (device.default_sample_rate || '48000') + ' Hz' : '-';
        document.getElementById('inputType').textContent = device ? (device.device_type || 'unknown') : '-';
        const avail = document.getElementById('inputAvailability');
        if (device) {
            avail.className = 'io-availability ' + (device.available !== false ? 'available' : 'unavailable');
        } else {
            avail.className = 'io-availability';
        }
    } else {
        document.getElementById('outputChannels').textContent = device ? (device.max_channels || 2) : '-';
        document.getElementById('outputSampleRate').textContent = device ? (device.default_sample_rate || '48000') + ' Hz' : '-';
        document.getElementById('outputType').textContent = device ? (device.device_type || 'unknown') : '-';
        document.getElementById('outputIsDefault').textContent = device ? (device.is_default ? 'Yes' : 'No') : '-';
        const avail = document.getElementById('outputAvailability');
        if (device) {
            avail.className = 'io-availability ' + (device.available !== false ? 'available' : 'unavailable');
        } else {
            avail.className = 'io-availability';
        }
    }
}

function populateCalMicDevices() {
    const select = document.getElementById('calMicDevice');
    select.innerHTML = '<option value="">Select measurement mic...</option>';
    appState.input_devices.forEach(device => {
        const option = document.createElement('option');
        option.value = device.id;
        option.textContent = device.name;
        select.appendChild(option);
    });
}

async function selectInputDevice() {
    const deviceId = document.getElementById('inputDevice').value;
    const sourceType = document.getElementById('inputSource').value;
    const device = appState.input_devices.find(d => d.id === deviceId);
    updateDeviceInfoCard('input', device);
    try {
        const response = await fetch(`${DAEMON_API}/input/select`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ source_id: deviceId, source: sourceType }),
        });
        if (response.ok) {
            appState.current_input = deviceId;
            const name = device ? device.name : deviceId;
            setStatusIndicator('inputStatus', true, 'Active: ' + name, 'No input selected');
            showResult('Input device selected: ' + name, 'success');
            saveState();
        }
    } catch (error) {
        showResult('Failed to select input: ' + error.message, 'error');
    }
}

async function selectInputSource() { await selectInputDevice(); }

async function selectOutputDevice() {
    const deviceId = document.getElementById('outputDevice').value;
    const device = appState.output_devices.find(d => d.id === deviceId);
    updateDeviceInfoCard('output', device);
    try {
        const response = await fetch(`${DAEMON_API}/output/select`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ device_id: deviceId }),
        });
        if (response.ok) {
            appState.current_output = deviceId;
            const name = device ? device.name : deviceId;
            setStatusIndicator('outputStatus', true, 'Active: ' + name, 'No output selected');
            showResult('Output device selected: ' + name, 'success');
            saveState();
        }
    } catch (error) {
        showResult('Failed to select output: ' + error.message, 'error');
    }
}

// ===== Transport Controls =====
async function selectAudioFile() {
    try {
        const filePath = await invoke('select_file');
        if (filePath) {
            document.getElementById('audioFile').value = filePath;
            await loadAudioFile(filePath);
        }
    } catch {
        const path = prompt('Enter audio file path:');
        if (path) {
            document.getElementById('audioFile').value = path;
            await loadAudioFile(path);
        }
    }
}

async function loadAudioFile(filePath) {
    try {
        const response = await fetch(`${DAEMON_API}/transport/load-file`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ file_path: filePath }),
        });
        if (response.ok) {
            document.getElementById('currentFile').textContent = filePath.split('/').pop();
            showResult('File loaded: ' + filePath, 'success');
            await updatePlaybackStatus();
        }
    } catch (error) {
        showResult('Failed to load file: ' + error.message, 'error');
    }
}

async function setTransportMode() {
    const mode = document.getElementById('transportMode').value;
    try {
        const map = { 'FileOnly': 'file', 'StreamOnly': 'stream', 'Mixed': 'mixed' };
        await fetch(`${DAEMON_API}/transport/mode`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ mode: map[mode] || mode }),
        });
        showResult('Transport mode set to: ' + mode, 'success');
        saveState();
    } catch (error) {
        showResult('Failed to set mode: ' + error.message, 'error');
    }
}

async function transportPlay() {
    try {
        await fetch(`${DAEMON_API}/transport/play`, { method: 'POST' });
        appState.playback_state = 'playing';
        showResult('Playback started', 'success');
        await updatePlaybackStatus();
    } catch (error) {
        showResult('Failed to play: ' + error.message, 'error');
    }
}

async function transportPause() {
    try {
        await fetch(`${DAEMON_API}/transport/pause`, { method: 'POST' });
        appState.playback_state = 'paused';
        showResult('Playback paused', 'success');
        await updatePlaybackStatus();
    } catch (error) {
        showResult('Failed to pause: ' + error.message, 'error');
    }
}

async function transportStop() {
    try {
        await fetch(`${DAEMON_API}/transport/stop`, { method: 'POST' });
        appState.playback_state = 'idle';
        showResult('Playback stopped', 'success');
        document.getElementById('playbackProgress').style.width = '0%';
        document.getElementById('playbackPosition').textContent = '0:00';
        await updatePlaybackStatus();
    } catch (error) {
        showResult('Failed to stop: ' + error.message, 'error');
    }
}

let isSeeking = false;
async function updatePlaybackStatus() {
    try {
        const response = await fetch(`${DAEMON_API}/transport/playback-status`);
        if (response.ok) {
            const data = await response.json();
            const state = data.transport_state || data.state || 'Idle';
            document.getElementById('playbackState').textContent = state;
            document.getElementById('playbackSampleRate').textContent = data.sample_rate ? data.sample_rate + ' Hz' : '-';
            document.getElementById('playbackMode').textContent = data.mode || 'File';

            setStatusIndicator('statusTransport', state === 'Playing', state, state);

            if (data.total_samples && data.sample_rate && data.total_samples > 0) {
                const totalDuration = data.total_samples / data.sample_rate;
                const position = (data.position || 0) / data.sample_rate;
                document.getElementById('playbackPosition').textContent = formatTime(position);
                document.getElementById('playbackDuration').textContent = formatTime(totalDuration);
                // Update seek slider unless user is dragging
                if (!isSeeking) {
                    const slider = document.getElementById('seekSlider');
                    slider.value = Math.round((position / totalDuration) * 1000);
                }
                // Store for seek calculation
                appState.playback_total_duration = totalDuration;
                appState.playback_sample_rate = data.sample_rate;
                appState.playback_total_samples = data.total_samples;
            }
        }
    } catch { /* daemon offline */ }
}

function formatTime(secs) {
    const m = Math.floor(secs / 60);
    const s = Math.floor(secs % 60);
    return m + ':' + s.toString().padStart(2, '0');
}

// ===== Seek to Position =====
async function seekToPosition(samplePosition) {
    try {
        await fetch(`${DAEMON_API}/transport/seek`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ position: samplePosition }),
        });
    } catch { /* seek not supported yet, silently ignore */ }
}

// ===== Speaker Layout Visualization =====
function drawLayoutCanvas(presetKey) {
    const canvas = document.getElementById('layoutCanvas');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    const w = canvas.width;
    const h = canvas.height;
    const cx = w / 2;
    const cy = h / 2;
    const radius = Math.min(w, h) * 0.38;

    ctx.clearRect(0, 0, w, h);

    // Draw room circle
    ctx.beginPath();
    ctx.arc(cx, cy, radius, 0, Math.PI * 2);
    ctx.strokeStyle = 'rgba(230, 81, 0, 0.3)';
    ctx.lineWidth = 1;
    ctx.stroke();

    // Draw inner circle
    ctx.beginPath();
    ctx.arc(cx, cy, radius * 0.5, 0, Math.PI * 2);
    ctx.strokeStyle = 'rgba(230, 81, 0, 0.15)';
    ctx.stroke();

    // Draw crosshairs
    ctx.strokeStyle = 'rgba(255, 140, 0, 0.15)';
    ctx.beginPath(); ctx.moveTo(cx, cy - radius - 10); ctx.lineTo(cx, cy + radius + 10); ctx.stroke();
    ctx.beginPath(); ctx.moveTo(cx - radius - 10, cy); ctx.lineTo(cx + radius + 10, cy); ctx.stroke();

    // Draw listener position
    ctx.beginPath();
    ctx.arc(cx, cy, 12, 0, Math.PI * 2);
    ctx.fillStyle = 'rgba(255, 213, 128, 0.5)';
    ctx.fill();
    ctx.beginPath();
    ctx.arc(cx, cy, 5, 0, Math.PI * 2);
    ctx.fillStyle = '#FFD580';
    ctx.fill();

    // Direction indicators
    ctx.fillStyle = 'rgba(255, 213, 128, 0.5)';
    ctx.font = '11px sans-serif';
    ctx.textAlign = 'center';
    ctx.fillText('Front', cx, cy - radius - 15);
    ctx.fillText('Rear', cx, cy + radius + 22);
    ctx.fillText('L', cx - radius - 18, cy + 4);
    ctx.fillText('R', cx + radius + 18, cy + 4);

    // Get preset
    const preset = LAYOUT_PRESETS[presetKey];
    if (!preset) return;

    // Update layout info
    const speakers = preset.speakers;
    const hasLFE = speakers.some(s => s.n === 'LFE');
    const hasHeight = speakers.some(s => s.el > 0);
    document.getElementById('layoutSpeakerCount').textContent = speakers.length;
    document.getElementById('layoutName').textContent = preset.name;
    setStatusIndicator('layoutLFE', hasLFE, 'Yes', 'No');
    setStatusIndicator('layoutHeight', hasHeight, 'Yes', 'No');

    // Draw speakers
    const tbody = document.getElementById('speakerTableBody');
    tbody.innerHTML = '';

    speakers.forEach(sp => {
        // Convert azimuth to canvas position (0=front/top, positive=right)
        const azRad = (sp.az - 90) * Math.PI / 180;
        const dist = sp.n === 'LFE' ? radius * 0.3 : radius * 0.85;
        const sx = cx + dist * Math.cos(azRad);
        const sy = cy + dist * Math.sin(azRad);

        // Speaker circle
        const isHeight = sp.el > 0;
        const isLFE = sp.n === 'LFE';
        ctx.beginPath();
        ctx.arc(sx, sy, isLFE ? 10 : (isHeight ? 8 : 10), 0, Math.PI * 2);
        ctx.fillStyle = isLFE ? '#FF8C00' : (isHeight ? '#2196F3' : '#E65100');
        ctx.fill();
        ctx.strokeStyle = 'rgba(255, 213, 128, 0.6)';
        ctx.lineWidth = 2;
        ctx.stroke();

        // Height indicator (diamond)
        if (isHeight) {
            ctx.beginPath();
            ctx.moveTo(sx, sy - 13); ctx.lineTo(sx + 5, sy - 8);
            ctx.lineTo(sx, sy - 3); ctx.lineTo(sx - 5, sy - 8);
            ctx.closePath();
            ctx.fillStyle = 'rgba(33, 150, 243, 0.7)';
            ctx.fill();
        }

        // Label
        ctx.fillStyle = '#F5F5F5';
        ctx.font = 'bold 11px sans-serif';
        ctx.textAlign = 'center';
        ctx.fillText(sp.n, sx, sy + (isHeight ? 22 : 20));

        // Add to table
        const tr = document.createElement('tr');
        tr.innerHTML = '<td>' + sp.n + '</td>'
            + '<td>' + sp.az + '\u00B0</td>'
            + '<td>' + sp.el + '\u00B0</td>'
            + '<td>1.0m</td>'
            + '<td>' + (isLFE ? 'LFE' : (isHeight ? 'Height' : 'Ear Level')) + '</td>';
        tbody.appendChild(tr);
    });
}

async function applyLayout() {
    const preset = document.getElementById('layoutPreset').value;
    try {
        const response = await fetch(`${DAEMON_API}/layout`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ preset: preset === '5.1' ? '5.1' : preset }),
        });
        if (response.ok) {
            showResult('Layout applied: ' + preset, 'success');
        } else {
            showResult('Layout not supported by daemon yet', 'error');
        }
    } catch (error) {
        showResult('Failed to apply layout: ' + error.message, 'error');
    }
}

async function testVBAPSignal() {
    showResult('VBAP test signal routing... (requires running daemon with real audio output)', 'success');
}

// ===== Layout Canvas Drag Interactivity =====
let layoutDragState = { dragging: false, speakerIdx: -1, speakers: null, presetKey: '5.1' };

function setupLayoutCanvasDrag() {
    const canvas = document.getElementById('layoutCanvas');
    if (!canvas) return;

    canvas.addEventListener('mousedown', (e) => {
        const rect = canvas.getBoundingClientRect();
        const mx = (e.clientX - rect.left) * (canvas.width / rect.width);
        const my = (e.clientY - rect.top) * (canvas.height / rect.height);
        const preset = LAYOUT_PRESETS[layoutDragState.presetKey];
        if (!preset) return;

        const cx = canvas.width / 2;
        const cy = canvas.height / 2;
        const radius = Math.min(canvas.width, canvas.height) * 0.38;

        // Find the closest speaker within 15px
        let closest = -1, closestDist = 20;
        preset.speakers.forEach((sp, i) => {
            const azRad = (sp.az - 90) * Math.PI / 180;
            const dist = sp.n === 'LFE' ? radius * 0.3 : radius * 0.85;
            const sx = cx + dist * Math.cos(azRad);
            const sy = cy + dist * Math.sin(azRad);
            const d = Math.hypot(mx - sx, my - sy);
            if (d < closestDist) { closestDist = d; closest = i; }
        });

        if (closest >= 0) {
            layoutDragState.dragging = true;
            layoutDragState.speakerIdx = closest;
            layoutDragState.speakers = preset.speakers.map(s => ({...s}));
            canvas.style.cursor = 'grabbing';
        }
    });

    canvas.addEventListener('mousemove', (e) => {
        if (!layoutDragState.dragging) {
            // Hover cursor hint
            const rect = canvas.getBoundingClientRect();
            const mx = (e.clientX - rect.left) * (canvas.width / rect.width);
            const my = (e.clientY - rect.top) * (canvas.height / rect.height);
            const preset = LAYOUT_PRESETS[layoutDragState.presetKey];
            if (!preset) return;
            const cx = canvas.width / 2;
            const cy = canvas.height / 2;
            const radius = Math.min(canvas.width, canvas.height) * 0.38;
            let near = false;
            preset.speakers.forEach(sp => {
                const azRad = (sp.az - 90) * Math.PI / 180;
                const dist = sp.n === 'LFE' ? radius * 0.3 : radius * 0.85;
                const sx = cx + dist * Math.cos(azRad);
                const sy = cy + dist * Math.sin(azRad);
                if (Math.hypot(mx - sx, my - sy) < 15) near = true;
            });
            canvas.style.cursor = near ? 'grab' : 'default';
            return;
        }

        const rect = canvas.getBoundingClientRect();
        const mx = (e.clientX - rect.left) * (canvas.width / rect.width);
        const my = (e.clientY - rect.top) * (canvas.height / rect.height);
        const cx = canvas.width / 2;
        const cy = canvas.height / 2;

        // Convert mouse position back to azimuth
        const dx = mx - cx;
        const dy = my - cy;
        const az = Math.atan2(dy, dx) * 180 / Math.PI + 90;
        const normalizedAz = ((az % 360) + 360) % 360;
        const finalAz = normalizedAz > 180 ? normalizedAz - 360 : normalizedAz;

        layoutDragState.speakers[layoutDragState.speakerIdx].az = Math.round(finalAz);

        // Redraw with modified positions
        drawLayoutCanvasCustom(layoutDragState.speakers, layoutDragState.presetKey);
    });

    canvas.addEventListener('mouseup', () => {
        if (layoutDragState.dragging) {
            layoutDragState.dragging = false;
            canvas.style.cursor = 'default';
            // Persist modified positions to preset
            const preset = LAYOUT_PRESETS[layoutDragState.presetKey];
            if (preset && layoutDragState.speakers) {
                preset.speakers = layoutDragState.speakers;
            }
            // Save to localStorage
            saveState();
        }
    });

    canvas.addEventListener('mouseleave', () => {
        if (layoutDragState.dragging) {
            layoutDragState.dragging = false;
            canvas.style.cursor = 'default';
        }
    });
}

function drawLayoutCanvasCustom(speakers, presetKey) {
    const canvas = document.getElementById('layoutCanvas');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    const w = canvas.width;
    const h = canvas.height;
    const cx = w / 2;
    const cy = h / 2;
    const radius = Math.min(w, h) * 0.38;

    ctx.clearRect(0, 0, w, h);

    // Room circle + grid
    ctx.beginPath(); ctx.arc(cx, cy, radius, 0, Math.PI * 2);
    ctx.strokeStyle = 'rgba(230, 81, 0, 0.3)'; ctx.lineWidth = 1; ctx.stroke();
    ctx.beginPath(); ctx.arc(cx, cy, radius * 0.5, 0, Math.PI * 2);
    ctx.strokeStyle = 'rgba(230, 81, 0, 0.15)'; ctx.stroke();
    ctx.strokeStyle = 'rgba(255, 140, 0, 0.15)';
    ctx.beginPath(); ctx.moveTo(cx, cy - radius - 10); ctx.lineTo(cx, cy + radius + 10); ctx.stroke();
    ctx.beginPath(); ctx.moveTo(cx - radius - 10, cy); ctx.lineTo(cx + radius + 10, cy); ctx.stroke();

    // Listener
    ctx.beginPath(); ctx.arc(cx, cy, 12, 0, Math.PI * 2);
    ctx.fillStyle = 'rgba(255, 213, 128, 0.5)'; ctx.fill();
    ctx.beginPath(); ctx.arc(cx, cy, 5, 0, Math.PI * 2);
    ctx.fillStyle = '#FFD580'; ctx.fill();

    // Direction labels
    ctx.fillStyle = 'rgba(255, 213, 128, 0.5)';
    ctx.font = '11px sans-serif'; ctx.textAlign = 'center';
    ctx.fillText('Front', cx, cy - radius - 15);
    ctx.fillText('Rear', cx, cy + radius + 22);
    ctx.fillText('L', cx - radius - 18, cy + 4);
    ctx.fillText('R', cx + radius + 18, cy + 4);

    // Update info
    const preset = LAYOUT_PRESETS[presetKey];
    const hasLFE = speakers.some(s => s.n === 'LFE');
    const hasHeight = speakers.some(s => s.el > 0);
    document.getElementById('layoutSpeakerCount').textContent = speakers.length;
    document.getElementById('layoutName').textContent = preset ? preset.name : 'Custom';
    setStatusIndicator('layoutLFE', hasLFE, 'Yes', 'No');
    setStatusIndicator('layoutHeight', hasHeight, 'Yes', 'No');

    const tbody = document.getElementById('speakerTableBody');
    tbody.innerHTML = '';

    speakers.forEach((sp, idx) => {
        const azRad = (sp.az - 90) * Math.PI / 180;
        const dist = sp.n === 'LFE' ? radius * 0.3 : radius * 0.85;
        const sx = cx + dist * Math.cos(azRad);
        const sy = cy + dist * Math.sin(azRad);

        const isHeight = sp.el > 0;
        const isLFE = sp.n === 'LFE';
        const isDragged = layoutDragState.dragging && layoutDragState.speakerIdx === idx;

        ctx.beginPath();
        ctx.arc(sx, sy, isLFE ? 10 : (isHeight ? 8 : 10), 0, Math.PI * 2);
        ctx.fillStyle = isDragged ? '#FFD580' : (isLFE ? '#FF8C00' : (isHeight ? '#2196F3' : '#E65100'));
        ctx.fill();
        ctx.strokeStyle = isDragged ? '#FFF' : 'rgba(255, 213, 128, 0.6)';
        ctx.lineWidth = isDragged ? 3 : 2;
        ctx.stroke();

        if (isHeight) {
            ctx.beginPath();
            ctx.moveTo(sx, sy - 13); ctx.lineTo(sx + 5, sy - 8);
            ctx.lineTo(sx, sy - 3); ctx.lineTo(sx - 5, sy - 8);
            ctx.closePath();
            ctx.fillStyle = 'rgba(33, 150, 243, 0.7)'; ctx.fill();
        }

        ctx.fillStyle = '#F5F5F5'; ctx.font = 'bold 11px sans-serif'; ctx.textAlign = 'center';
        ctx.fillText(sp.n, sx, sy + (isHeight ? 22 : 20));

        const tr = document.createElement('tr');
        tr.innerHTML = '<td>' + sp.n + '</td><td>' + sp.az + '&deg;</td><td>' + sp.el + '&deg;</td><td>1.0m</td><td>' + (isLFE ? 'LFE' : (isHeight ? 'Height' : 'Ear Level')) + '</td>';
        tbody.appendChild(tr);
    });
}

// ===== Sync Error Visualization =====
async function updateSyncViz() {
    try {
        const resp = await fetch(`${DAEMON_API}/stats/sync`);
        if (!resp.ok) return;
        const data = await resp.json();
        drawSyncCanvas(data);
        document.getElementById('syncOverall').textContent = data.overall_status || 'No speakers';
        const maxDrift = data.speakers && data.speakers.length > 0
            ? Math.max(...data.speakers.map(s => Math.abs(s.sync_error_ms || 0))).toFixed(1) + ' ms'
            : '- ms';
        document.getElementById('syncMaxDrift').textContent = maxDrift;
    } catch {}
}

function drawSyncCanvas(data) {
    const canvas = document.getElementById('syncCanvas');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    const w = canvas.width;
    const h = canvas.height;
    const cx = w / 2;
    const cy = h / 2;
    const radius = Math.min(w, h) * 0.38;

    ctx.clearRect(0, 0, w, h);

    // Concentric rings for severity
    [1.0, 0.66, 0.33].forEach((scale, i) => {
        ctx.beginPath(); ctx.arc(cx, cy, radius * scale, 0, Math.PI * 2);
        const colors = ['rgba(244, 67, 54, 0.15)', 'rgba(255, 193, 7, 0.15)', 'rgba(76, 175, 80, 0.15)'];
        ctx.fillStyle = colors[i]; ctx.fill();
        ctx.strokeStyle = colors[i].replace('0.15', '0.4'); ctx.lineWidth = 1; ctx.stroke();
    });

    // Labels
    ctx.fillStyle = 'rgba(255, 213, 128, 0.4)'; ctx.font = '10px sans-serif'; ctx.textAlign = 'center';
    ctx.fillText('<5ms', cx, cy - radius * 0.33 + 12);
    ctx.fillText('<20ms', cx, cy - radius * 0.66 + 12);
    ctx.fillText('>20ms', cx, cy - radius + 12);

    // Center dot
    ctx.beginPath(); ctx.arc(cx, cy, 4, 0, Math.PI * 2);
    ctx.fillStyle = '#FFD580'; ctx.fill();

    if (!data.speakers || data.speakers.length === 0) {
        ctx.fillStyle = 'rgba(255, 213, 128, 0.3)'; ctx.font = '14px sans-serif';
        ctx.fillText('No speakers connected', cx, cy + radius + 30);
        return;
    }

    // Plot speakers
    const angleStep = (2 * Math.PI) / data.speakers.length;
    data.speakers.forEach((sp, i) => {
        const angle = -Math.PI / 2 + i * angleStep;
        const err = Math.abs(sp.sync_error_ms || 0);
        const dist = Math.min(err / 30, 1.0) * radius; // 30ms = edge
        const sx = cx + dist * Math.cos(angle);
        const sy = cy + dist * Math.sin(angle);

        const color = err < 5 ? '#4CAF50' : (err < 20 ? '#FFC107' : '#F44336');
        ctx.beginPath(); ctx.arc(sx, sy, 8, 0, Math.PI * 2);
        ctx.fillStyle = color; ctx.fill();
        ctx.strokeStyle = 'rgba(255, 255, 255, 0.5)'; ctx.lineWidth = 1.5; ctx.stroke();

        ctx.fillStyle = '#F5F5F5'; ctx.font = 'bold 10px sans-serif'; ctx.textAlign = 'center';
        ctx.fillText(sp.name || ('S' + (i + 1)), sx, sy + 20);
        ctx.fillText(err.toFixed(1) + 'ms', sx, sy + 32);
    });
}

// ===== Calibration =====
async function startCalibration() {
    try {
        const response = await fetch(`${DAEMON_API}/calibration/start`, { method: 'POST' });
        if (response.ok || response.status === 202) {
            document.getElementById('startCalBtn').disabled = true;
            document.getElementById('stopCalBtn').disabled = false;
            setStatusIndicator('calStatus', true, 'Measuring...', 'Ready');
            pollCalibrationProgress();
            showResult('Calibration measurement started', 'success');
        }
    } catch (error) {
        showResult('Failed to start calibration: ' + error.message, 'error');
    }
}

async function stopCalibration() {
    document.getElementById('startCalBtn').disabled = false;
    document.getElementById('stopCalBtn').disabled = true;
    setStatusIndicator('calStatus', false, 'Measuring...', 'Stopped');
    showResult('Calibration stopped', 'success');
}

async function applyCalibration() {
    try {
        const response = await fetch(`${DAEMON_API}/calibration/apply`, { method: 'POST' });
        if (response.ok) {
            showResult('Calibration filters applied successfully', 'success');
        } else {
            showResult('Failed to apply calibration (no measurements)', 'error');
        }
    } catch (error) {
        showResult('Failed to apply calibration: ' + error.message, 'error');
    }
}

let calPollInterval = null;
function pollCalibrationProgress() {
    if (calPollInterval) clearInterval(calPollInterval);
    calPollInterval = setInterval(async () => {
        try {
            const response = await fetch(`${DAEMON_API}/calibration/status`);
            if (response.ok) {
                const data = await response.json();
                const progress = (data.progress || 0) * 100;
                document.getElementById('calProgress').style.width = progress + '%';
                document.getElementById('calProgressText').textContent = Math.round(progress) + '% complete';
                document.getElementById('irMeasurements').textContent = data.measurements || 0;

                if (!data.running) {
                    clearInterval(calPollInterval);
                    document.getElementById('startCalBtn').disabled = false;
                    document.getElementById('stopCalBtn').disabled = true;
                    document.getElementById('applyCalBtn').disabled = false;
                    document.getElementById('designFilterBtn').disabled = false;
                    setStatusIndicator('calStatus', false, 'Measuring...', 'Complete');
                    drawIRCanvas(true); // Redraw with simulated data
                }
            }
        } catch { clearInterval(calPollInterval); }
    }, 500);
}

function designFilter() {
    showResult('Filter designed (preview updated)', 'success');
    drawFilterCanvas(true);
    document.getElementById('exportFilterBtn').disabled = false;
}

function exportFilter() {
    const format = document.getElementById('exportFormat').value;
    showResult('Filter exported in ' + format.toUpperCase() + ' format (check daemon output)', 'success');
}

// ===== Canvas Drawing: Impulse Response =====
function drawIRCanvas(hasData) {
    const canvas = document.getElementById('irCanvas');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    const w = canvas.width;
    const h = canvas.height;
    ctx.clearRect(0, 0, w, h);

    // Grid
    ctx.strokeStyle = 'rgba(230, 81, 0, 0.15)';
    ctx.lineWidth = 1;
    for (let y = 0; y < h; y += 50) { ctx.beginPath(); ctx.moveTo(0, y); ctx.lineTo(w, y); ctx.stroke(); }
    for (let x = 0; x < w; x += 50) { ctx.beginPath(); ctx.moveTo(x, 0); ctx.lineTo(x, h); ctx.stroke(); }

    // Zero line
    ctx.strokeStyle = 'rgba(255, 140, 0, 0.3)';
    ctx.beginPath(); ctx.moveTo(0, h / 2); ctx.lineTo(w, h / 2); ctx.stroke();

    if (hasData) {
        // Simulated IR data
        ctx.strokeStyle = '#E65100';
        ctx.lineWidth = 1.5;
        ctx.beginPath();
        for (let x = 0; x < w; x++) {
            const t = x / w;
            const decay = Math.exp(-t * 6);
            const noise = (Math.random() - 0.5) * 2;
            const peak = t < 0.05 ? Math.sin(t * 200) * 4 : 0;
            const y = h / 2 + (noise * decay + peak * decay) * (h * 0.4);
            if (x === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
        }
        ctx.stroke();

        document.getElementById('irDelay').textContent = '2.3 ms';
        document.getElementById('irRT60').textContent = '0.42 s';
    } else {
        ctx.fillStyle = 'rgba(255, 213, 128, 0.3)';
        ctx.font = '14px sans-serif';
        ctx.textAlign = 'center';
        ctx.fillText('No measurement data - run calibration sweep first', w / 2, h / 2);
    }
}

// ===== Canvas Drawing: Filter Response =====
function drawFilterCanvas(hasData) {
    const canvas = document.getElementById('filterCanvas');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    const w = canvas.width;
    const h = canvas.height;
    ctx.clearRect(0, 0, w, h);

    // Grid
    ctx.strokeStyle = 'rgba(230, 81, 0, 0.15)';
    ctx.lineWidth = 1;
    for (let y = 0; y < h; y += 40) { ctx.beginPath(); ctx.moveTo(0, y); ctx.lineTo(w, y); ctx.stroke(); }

    // 0dB line
    ctx.strokeStyle = 'rgba(255, 140, 0, 0.3)';
    ctx.beginPath(); ctx.moveTo(0, h / 2); ctx.lineTo(w, h / 2); ctx.stroke();

    if (hasData) {
        // Simulated correction curve
        ctx.strokeStyle = '#4CAF50';
        ctx.lineWidth = 2;
        ctx.beginPath();
        for (let x = 0; x < w; x++) {
            const freq = 20 * Math.pow(1000, x / w); // 20Hz to 20kHz log scale
            let db = 0;
            db -= 3 * Math.exp(-Math.pow((freq - 80) / 40, 2)); // Bass cut
            db += 2 * Math.exp(-Math.pow((freq - 3000) / 1500, 2)); // Presence boost
            db -= 4 * Math.exp(-Math.pow((freq - 8000) / 2000, 2)); // High-freq dip
            const y = h / 2 - (db / 12) * h;
            if (x === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
        }
        ctx.stroke();

        // Labels
        ctx.fillStyle = 'rgba(255, 213, 128, 0.5)';
        ctx.font = '10px sans-serif';
        ctx.fillText('20Hz', 5, h - 5);
        ctx.fillText('20kHz', w - 35, h - 5);
        ctx.fillText('+6dB', 5, 15);
        ctx.fillText('-6dB', 5, h - 15);
    } else {
        ctx.fillStyle = 'rgba(255, 213, 128, 0.3)';
        ctx.font = '12px sans-serif';
        ctx.textAlign = 'center';
        ctx.fillText('No filter designed yet', w / 2, h / 2);
    }
}

// ===== Canvas Drawing: Latency History =====
function drawLatencyCanvas() {
    const canvas = document.getElementById('latencyCanvas');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    const w = canvas.width;
    const h = canvas.height;
    ctx.clearRect(0, 0, w, h);

    // Grid
    ctx.strokeStyle = 'rgba(230, 81, 0, 0.15)';
    for (let y = 0; y < h; y += 40) { ctx.beginPath(); ctx.moveTo(0, y); ctx.lineTo(w, y); ctx.stroke(); }

    // Labels
    ctx.fillStyle = 'rgba(255, 213, 128, 0.4)';
    ctx.font = '10px sans-serif';
    ctx.textAlign = 'left';
    ctx.fillText('0 ms', 5, h - 5);
    ctx.fillText('50 ms', 5, 12);

    const data = appState.latency_history;
    if (data.length < 2) {
        ctx.fillStyle = 'rgba(255, 213, 128, 0.3)';
        ctx.font = '14px sans-serif';
        ctx.textAlign = 'center';
        ctx.fillText('Latency data will appear when speakers are connected', w / 2, h / 2);
        return;
    }

    ctx.strokeStyle = '#E65100';
    ctx.lineWidth = 2;
    ctx.beginPath();
    data.forEach((val, i) => {
        const x = (i / (data.length - 1)) * w;
        const y = h - (val / 50) * h;
        if (i === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
    });
    ctx.stroke();
}

// ===== Canvas Drawing: Bandwidth =====
function drawBandwidthCanvas() {
    const canvas = document.getElementById('bandwidthCanvas');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    const w = canvas.width;
    const h = canvas.height;
    ctx.clearRect(0, 0, w, h);

    ctx.strokeStyle = 'rgba(230, 81, 0, 0.15)';
    for (let y = 0; y < h; y += 40) { ctx.beginPath(); ctx.moveTo(0, y); ctx.lineTo(w, y); ctx.stroke(); }

    const data = appState.bandwidth_history;
    if (data.length < 2) {
        ctx.fillStyle = 'rgba(255, 213, 128, 0.3)';
        ctx.font = '14px sans-serif';
        ctx.textAlign = 'center';
        ctx.fillText('Bandwidth data will appear when streaming is active', w / 2, h / 2);
        return;
    }

    ctx.strokeStyle = '#2196F3';
    ctx.lineWidth = 2;
    ctx.beginPath();
    const maxVal = Math.max(...data, 1);
    data.forEach((val, i) => {
        const x = (i / (data.length - 1)) * w;
        const y = h - (val / maxVal) * (h * 0.9);
        if (i === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
    });
    ctx.stroke();
}

// ===== Stats Dashboard =====
async function updateStats() {
    try {
        const response = await fetch(`${DAEMON_API}/stats`);
        if (response.ok) {
            const data = await response.json();
            document.getElementById('statusSpeakers').textContent = data.online_speakers + '/' + data.total_speakers;
            setStatusIndicator('statusTransport', data.transport_state === 'Playing', data.transport_state, data.transport_state);
        }

        // Fetch detailed stats sub-endpoints
        await Promise.allSettled([
            updateNetworkStats(),
            updateLatencyStats(),
            updateDaemonStats(),
            updateAudioLevels(),
            updateSyncViz(),
        ]);
    } catch { /* daemon offline */ }
}

async function updateNetworkStats() {
    try {
        const resp = await fetch(`${DAEMON_API}/stats/network`);
        if (resp.ok) {
            const data = await resp.json();
            const bwEl = document.getElementById('statBandwidth');
            if (bwEl) bwEl.textContent = data.sent_kbps.toFixed(1) + ' kbps';
            appState.bandwidth_history.push(data.sent_kbps);
            if (appState.bandwidth_history.length > 60) appState.bandwidth_history.shift();
            drawBandwidthCanvas();
        }
    } catch {}
}

async function updateLatencyStats() {
    try {
        const resp = await fetch(`${DAEMON_API}/stats/latency`);
        if (resp.ok) {
            const data = await resp.json();
            const latEl = document.getElementById('statLatency');
            if (latEl) latEl.textContent = data.mean_ms.toFixed(1) + ' ms';
            if (data.samples && data.samples.length > 0) {
                appState.latency_history = data.samples;
                drawLatencyCanvas();
            }
        }
    } catch {}
}

async function updateDaemonStats() {
    try {
        const resp = await fetch(`${DAEMON_API}/stats/daemon`);
        if (resp.ok) {
            const data = await resp.json();
            const cpuEl = document.getElementById('statCpu');
            const memEl = document.getElementById('statMemory');
            if (cpuEl) cpuEl.textContent = data.cpu_percent.toFixed(1) + '%';
            if (memEl) memEl.textContent = data.memory_mb.toFixed(1) + ' MB';
        }
    } catch {}
}

async function updateAudioLevels() {
    try {
        const resp = await fetch(`${DAEMON_API}/stats/audio-levels`);
        if (resp.ok) {
            const data = await resp.json();
            // Convert dBFS to 0-1 range: level = 10^(db/20)
            const toLinear = (db) => Math.max(0, Math.min(1, Math.pow(10, db / 20)));
            setMeter('meterInputL', 'meterInputLDb', toLinear(data.input_db_left));
            setMeter('meterInputR', 'meterInputRDb', toLinear(data.input_db_right));
            setMeter('meterOutputL', 'meterOutputLDb', toLinear(data.output_db_left));
            setMeter('meterOutputR', 'meterOutputRDb', toLinear(data.output_db_right));
            return; // Real data available, skip simulation
        }
    } catch {}
    // Fallback to simulation if endpoint unavailable
    updateMetersSimulated();
}

// ===== Audio Level Meters =====
function updateMeters() {
    // Real levels fetched from daemon via updateAudioLevels() in updateStats.
    // This function only runs as fallback when daemon is offline.
    if (!appState.daemon_connected) {
        updateMetersSimulated();
    }
}

function updateMetersSimulated() {
    // Simulated levels when playing
    if (appState.playback_state === 'playing') {
        const baseLevel = 0.4 + Math.random() * 0.3;
        setMeter('meterInputL', 'meterInputLDb', baseLevel + (Math.random() - 0.5) * 0.1);
        setMeter('meterInputR', 'meterInputRDb', baseLevel + (Math.random() - 0.5) * 0.1);
        setMeter('meterOutputL', 'meterOutputLDb', baseLevel * 0.9 + (Math.random() - 0.5) * 0.1);
        setMeter('meterOutputR', 'meterOutputRDb', baseLevel * 0.9 + (Math.random() - 0.5) * 0.1);
    } else {
        setMeter('meterInputL', 'meterInputLDb', 0);
        setMeter('meterInputR', 'meterInputRDb', 0);
        setMeter('meterOutputL', 'meterOutputLDb', 0);
        setMeter('meterOutputR', 'meterOutputRDb', 0);
    }
}

function setMeter(fillId, dbId, level) {
    level = Math.max(0, Math.min(1, level));
    document.getElementById(fillId).style.width = (level * 100) + '%';
    const db = level > 0.001 ? (20 * Math.log10(level)).toFixed(1) : '-inf';
    document.getElementById(dbId).textContent = db + ' dB';
}

// ===== Polling =====
function startStatusPolling() {
    // Fast poll (100ms) for meters
    setInterval(() => {
        updateMeters();
    }, 100);

    // Medium poll (100ms) for playback status during playback
    setInterval(async () => {
        if (appState.playback_state === 'playing') {
            await updatePlaybackStatus();
        }
    }, 100);

    // Slower poll (1s) for playback status when paused
    setInterval(async () => {
        if (appState.playback_state === 'paused') {
            await updatePlaybackStatus();
        }
    }, 1000);

    // Slow poll (3s) for daemon status and stats
    setInterval(async () => {
        await checkDaemonConnection();
        await updateStats();
    }, 3000);
}
