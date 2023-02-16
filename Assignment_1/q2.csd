<CsoundSynthesizer>
<CsOptions>
-odac -m128
</CsOptions>
<CsInstruments>
sr = 44100
nchnls = 2
0dbfs = 1
ksmps = 32

gi_Arr_1[] fillarray 60, 62, 64, 65, 67, 69, 71, 72

instr Select
    iArr[] = gi_Arr_1
    index = 0
    while index < lenarray(iArr) do
  schedule("Play",index/2,1,iArr[index])
  index += 1
 od
endin

instr Play
 aImp mpulse 1, p3
 iFreq = mtof:i(p4)
 aTone mode aImp,iFreq,100
 out aTone, aTone
endin

</CsInstruments>
<CsScore>
i "Select" 0 4 
</CsScore>
</CsoundSynthesizer>