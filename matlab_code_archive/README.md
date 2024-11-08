# Matlab code archive

This code is from [Dan Ellis' webpage](https://www.ee.columbia.edu/~dpwe/resources/matlab/sws/).
The original README follows, with minor formatting edits:

## README for ~dpwe/matlab/sws-1996aug

dpwe@icsi.berkeley.edu 1996aug23

This directory contains functions for regenerating sinewave speech
from the parameters published by Philip Rubin of Haskins Labs at

  http://www.haskins.yale.edu/Haskins/MISC/SWS/sentences.html

Sinewave speech is an unexpectedly intelligible analog of speech
where three or four formant tracks are reproduced by sine tones
alone.  That these combinations evoke a phonetic perception raises
very deep questions about the nature of auditory organization.

You can reproduce the sound examples on the web page, as well as
experimenting with manipulations such as removing or altering
certain components, using the functions in this directory.
The *.swi files define the frequency and amplitudes for the
formant tones, and are downloaded from the web site (there are
nine examples there, S1pars.swi - S9pars.swi).

This directory contains three matlab scripts:

```
  X = synthtrax(F, M, SR, SUBF, DUR)      Reconstruct a sound from track rep'n.
        Each row of F and M contains a series of frequency and magnitude
        samples for a particular track.  These will be remodulated and
        overlaid into the output sound X which will run at sample rate SR,
        although the columns in F and M are subsampled from that rate by
        a factor SUBF.  If DUR is nonzero, X will be padded or truncated
        to correspond to just this much time.

  Y = slinterp(X,F)  Simple linear-interpolate X by a factor F
         Y will have ((size(X)-1)*F)+1 points i.e. no extrapolation

  [F,M] = readswi(NAME)  Read a Haskins-format sinewave speech data file
      NAME is the name of a text data file containing the frequency
      and magnitude parameters for sinewave synthesis.  Result is
      F and M matrices suitable for synthtrax.m
```

You use them like this (within matlab):

```matlab
>>   % Read in arrays defining the frequency and magnitude of the oscillators:
>> [F,M] = readswi('S1pars.swi');
>>   % Each row of F and M defines a single oscillator.  Columns are uniformly
>>   % spaced time samples.
>>   % Synthesize an audio signal based on the parameters:
>> X = synthtrax(F,M,8000,80);    % Takes 1.05s on Sparc5, 17s on Duo270c
>>   % X is the output of oscillators controlled by F and M.  Its sampling
>>   % rate is 8000 Hz, and the control columns were interpolated by a
>>   % factor of 80 before synthesis (i.e. 100 Hz control rate).
>>   % Play the sound at 8000 Hz SR (on a sun, mac, sgi...):
>> sound(X,8000)
>>   % We can also synthesize each track separately by selecting rows
>>   % in F and M:
>> T1 = synthtrax(F(1,:), M(1,:), 8000, 80);
>> T2 = synthtrax(F(2,:), M(2,:), 8000, 80);
>> T3 = synthtrax(F(3,:), M(3,:), 8000, 80);
>>   % Listen to combinations of the sine tones:
>> sound(T1, 8000);
>> sound(T1+T2, 8000);
>> sound(T1+T3, 8000);
>> sound(T1+T2+T3, 8000);
>>   % etc...
>>   % We can even look at the oscillator tracks:
>> plot(F')
>>   % Maybe force the frequency to zero when magnitude is zero
>> plot((F.*(M~=0))')
```

This code has been tried under Matlab 4.2c running on a Solaris 2.5
SPARCstation and Matlab 4.1 running on a MacOS 7.5.1 Powerbook Duo 270c.

Robert Remez, Philip Rubin and others have published a large series of
papers on the nature of this strange phenomenon (the references are at
the web site mentioned above).  You will be able to reproduce their
stimuli, including reversed formant tracks, dichotic presentation
(given a stereo playback extension such as SoundMex4.1) etc.  Have
fun.
