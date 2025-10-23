use crate::audio::AudioDetails;

struct SongView<'a> {
    audio_details_list: &'a[AudioDetails],
    selected: usize,
}

// impl SongView<'a> {
//     fn new(audio_details_list: &'a [AudioDetails], selected: usize) -> Self {
//         Self { audio_details_list, selected }
//     }
// }
//
