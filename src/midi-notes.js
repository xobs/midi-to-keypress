var notes = ["C", "Cs", "D", "Ds", "E", "F", "Fs", "G", "Gs", "A", "As", "B"];

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