<CsOptions>
; uncomment the next line if you want to play through speakers
-odac -Ma --midi-key-cps=4 --midi-velocity-amp=5
</CsOptions>
<CsoundSynthesizer>
<CsInstruments>

; the next line sets the volume scale 0-1
; by default this value is 32767
0dbfs = 1

; defines the first instrument
instr 1
; variable for output,  instrument type,  amplitude,  pitch input 
  iamp = 44100
  icps = 440
  iphs = 0
; 
  a1 oscils iamp, icps, iphs
  out a1
endin


</CsInstruments>
<CsScore>
; plays three notes in succession
; instrument  time to play at   length to play  frequency to play
i 1 0 1 60
i . + 1 >
i . + 1 >
i . + 1 >
i . + 1 100
</CsScore>
</CsoundSynthesizer>
