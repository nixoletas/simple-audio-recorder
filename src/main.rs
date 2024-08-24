use record_audio::audio_clip::AudioClip as ac;

fn main() {
    match ac::record(None) {
        Ok(clip) => {
            println!("Successfully recorded!");
            
            // Make sure the clip name is sanitized
            let sanitized_name = sanitize_filename::sanitize(&clip.name);
            let file_name = format!("{}.wav", sanitized_name);

            match clip.export(&file_name) {
                Ok(_) => {
                    println!("Successfully saved as {}", file_name);
                    
                    // Play immediately, after ctrl-c
                    clip.play().unwrap();
                }
                Err(err) => println!("Error exporting: {}", err),
            }
        }
        Err(err) => println!("Error recording: {}", err),
    }
}
