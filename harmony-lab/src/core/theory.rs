use eframe::egui;

pub struct ScalePattern {
    pub name: &'static str,
    pub intervals: &'static [usize],
}

pub const NOTE_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

pub const CIRCLE_OF_FIFTHS: [usize; 12] = [0, 7, 2, 9, 4, 11, 6, 1, 8, 3, 10, 5];

pub const ACCIDENTALS: [&str; 12] = [
    "0", "1#", "2#", "3#", "4#", "5#", "6# / 6b", "5b", "4b", "3b", "2b", "1b",
];

pub const DEFAULT_COLORS: [egui::Color32; 12] = [
    egui::Color32::from_rgb(210, 43, 43),   // 0: Do
    egui::Color32::from_rgb(129, 19, 49),   // 1: Do#
    egui::Color32::from_rgb(255, 127, 80),   // 2: Re
    egui::Color32::from_rgb(184, 115, 51),   // 3: Re#
    egui::Color32::from_rgb(255, 191, 0),   // 4: Mi
    egui::Color32::from_rgb(175, 225, 175),   // 5: Fa
    egui::Color32::from_rgb(9, 121, 105),   // 6: Fa#
    egui::Color32::from_rgb(100, 149, 237),   // 7: Sol
    egui::Color32::from_rgb(25, 25, 112),   // 8: Sol#
    egui::Color32::from_rgb(218, 112, 214),   // 9: La
    egui::Color32::from_rgb(93, 63, 211),   // 10: La#
    egui::Color32::from_rgb(204, 204, 255),   // 11: Si
];

pub const SCALES: &[ScalePattern] = &[
    ScalePattern { name: "None (Standard)", intervals: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11] },
    ScalePattern { name: "Major (Ionian)", intervals: &[0, 2, 4, 5, 7, 9, 11] },
    ScalePattern { name: "Dorian", intervals: &[0, 2, 3, 5, 7, 9, 10] },
    ScalePattern { name: "Phrygian", intervals: &[0, 1, 3, 5, 7, 8, 10] },
    ScalePattern { name: "Lydian", intervals: &[0, 2, 4, 6, 7, 9, 11] },
    ScalePattern { name: "Mixolydian", intervals: &[0, 2, 4, 5, 7, 9, 10] },
    ScalePattern { name: "Minor (Aeolian)", intervals: &[0, 2, 3, 5, 7, 8, 10] },
    ScalePattern { name: "Locrian", intervals: &[0, 1, 3, 5, 6, 8, 10] },
    ScalePattern { name: "Harmonic Minor", intervals: &[0, 2, 3, 5, 7, 8, 11] },
    ScalePattern { name: "Locrian Natural 6", intervals: &[0, 1, 3, 5, 6, 9, 10] },
    ScalePattern { name: "Ionian Augmented", intervals: &[0, 2, 4, 5, 8, 9, 11] },
    ScalePattern { name: "Dorian #4", intervals: &[0, 2, 3, 6, 7, 9, 10] },
    ScalePattern { name: "Phrygian Dominant", intervals: &[0, 1, 4, 5, 7, 8, 10] },
    ScalePattern { name: "Lydian #2", intervals: &[0, 3, 4, 6, 7, 9, 11] },
    ScalePattern { name: "Altered Diminished", intervals: &[0, 1, 3, 4, 6, 8, 9] },
    ScalePattern { name: "Melodic Minor", intervals: &[0, 2, 3, 5, 7, 9, 11] },
    ScalePattern { name: "Lydian b7 (Acoustic)", intervals: &[0, 2, 4, 6, 7, 9, 10] },
    ScalePattern { name: "Superlocrian (Altered)", intervals: &[0, 1, 3, 4, 6, 8, 10] },
    ScalePattern { name: "Diminished (W-H)", intervals: &[0, 2, 3, 5, 6, 8, 9, 11] },
    ScalePattern { name: "Whole Tone", intervals: &[0, 2, 4, 6, 8, 10] },
    ScalePattern { name: "Major Pentatonic", intervals: &[0, 2, 4, 7, 9] },
    ScalePattern { name: "Minor Pentatonic", intervals: &[0, 3, 5, 7, 10] },
    ScalePattern { name: "Japanese (Hirajoshi)", intervals: &[0, 2, 3, 7, 8] },
];

#[derive(PartialEq, Clone, Copy)]
pub enum ChordPattern {
    Triad, Seventh, Ninth, Eleventh, Thirteenth,
    Sus2, Sus4, SevenSus2, SevenSus4,
    Add9, Add11, Add13,
    PowerChord, Quartal3, Quartal4, Cluster,
}

impl ChordPattern {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Triad => "Triad (1-3-5)",
            Self::Seventh => "Seventh (1-3-5-7)",
            Self::Ninth => "Ninth (1-3-5-7-9)",
            Self::Eleventh => "Eleventh (1-3-5-7-9-11)",
            Self::Thirteenth => "Thirteenth (1-3-5-7-9-11-13)",
            Self::Sus2 => "Sus2 (1-2-5)",
            Self::Sus4 => "Sus4 (1-4-5)",
            Self::SevenSus2 => "7Sus2 (1-2-5-7)",
            Self::SevenSus4 => "7Sus4 (1-4-5-7)",
            Self::Add9 => "Add9 (1-3-5-9)",
            Self::Add11 => "Add11 (1-3-5-11)",
            Self::Add13 => "Add13 (1-3-5-13)",
            Self::PowerChord => "Power Chord (1-5)",
            Self::Quartal3 => "Quartal 3-part (1-4-7)",
            Self::Quartal4 => "Quartal 4-part (1-4-7-10)",
            Self::Cluster => "Cluster (1-2-3)",
        }
    }

    pub fn intervals(&self) -> Vec<usize> {
        match self {
            Self::Triad => vec![0, 2, 4],
            Self::Seventh => vec![0, 2, 4, 6],
            Self::Ninth => vec![0, 2, 4, 6, 1],
            Self::Eleventh => vec![0, 2, 4, 6, 1, 3],
            Self::Thirteenth => vec![0, 2, 4, 6, 1, 3, 5],
            Self::Sus2 => vec![0, 1, 4],
            Self::Sus4 => vec![0, 3, 4],
            Self::SevenSus2 => vec![0, 1, 4, 6],
            Self::SevenSus4 => vec![0, 3, 4, 6],
            Self::Add9 => vec![0, 2, 4, 1],
            Self::Add11 => vec![0, 2, 4, 3],
            Self::Add13 => vec![0, 2, 4, 5],
            Self::PowerChord => vec![0, 4],
            Self::Quartal3 => vec![0, 3, 6],
            Self::Quartal4 => vec![0, 3, 6, 2],
            Self::Cluster => vec![0, 1, 2],
        }
    }
}