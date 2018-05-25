var notes = ["C", "Cs", "D", "Ds", "E", "F", "Fs", "G", "Gs", "A", "As", "B"];
var keys = ['Return',
'Tab',
'Space',
'Backspace',
'Escape',
'Super',
'Command',
'Windows',
'Shift',
'CapsLock',
'Alt',
'Option',
'Control',
'Home',
'PageUp',
'PageDown',
'LeftArrow',
'RightArrow',
'DownArrow',
'UpArrow',
'F1',
'F2',
'F3',
'F4',
'F5',
'F6',
'F7',
'F8',
'F9',
'F10',
'F11',
'F12'];

function numberToNote(num) {
    var baseNote = notes[num%12];
    var octave = parseInt((num - 12) / 12);
    if (num < 12) {
        octave = "n";
    }
    return baseNote + octave;
}

console.log('pub enum MidiNote {');
for (var i = 0; i < 128; i++) {
    console.log('    ' + numberToNote(i) + ' = ' + i + ",");
}
console.log('}');

console.log('');
console.log('    pub fn new_from_text(txt: &str) -> Result<MidiNote, MidiError> {');
console.log('        let s = txt.to_lowercase();');
console.log('        if s.starts_with("#") { Err(MidiError::Unparseable) }');
for (var i = 0; i < 128; i++) {
    console.log('        else if s.starts_with("' + numberToNote(i).toLowerCase() + '") { Ok(MidiNote::' + numberToNote(i) + ') }');
}
console.log('        else {Err(MidiError::Unparseable) }');
console.log('    }');

keys.forEach((val, idx) => {
    console.log('    ' + val);
})