pub enum AudioDeviceID {
    PlaybackDevice(sys::SDL_AudioDeviceID),
}

impl AudioDeviceID {
    pub fn id(&self) -> sys::SDL_AudioDeviceID {
        match *self {
            AudioDeviceID::PlaybackDevice(id) => id,
        }
    }
}

impl Drop for AudioDeviceID {
    #[doc(alias = "SDL_CloseAudioDevice")]
    fn drop(&mut self) {
        //! Shut down audio processing and close the audio device.
        unsafe { sys::SDL_CloseAudioDevice(self.id()) }
    }
}
