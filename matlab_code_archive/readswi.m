function [F,M] = readswi(NAME)
% [F,M] = readswi(NAME)  Read a Haskins-format sinewave speech data file
%     NAME is the name of a text data file containing the frequency 
%     and magnitude parameters for sinewave synthesis.  Result is 
%     F and M matrices suitable for synthtrax.m
% dpwe@icsi.berkeley.edu 1996aug22

% SWI files are downloaded from 
%    http://www.haskins.yale.edu/Haskins/MISC/SWS/sentences.html
% and have the format:
%    Number of oscillators
%      Time0
%         frq,mag   for 1st oscillator
%         frq,mag   for 2nd oscillator
%         .. for as many oscillators as specified
%      Time1
%         frq,mag  ... etc.
% Times are in ms, frq in Hz, mag in linear units
%
% Here, we're assuming the times are uniformly spaced, and it is 
% up to the user to know the correct interpolation factor to 
% give to synthtrax.
%
% BE SURE TO TRIM OFF THE TEXT AT THE TOP AND BOTTOM OF THE FILES
% IF YOU SAVE DIRECTLY FROM THE WEB PAGES!

colchunk = 100;
col = 0;

fid = fopen(NAME, 'r');
if (fid == -1)
  fprintf(1, 'readswi: unable to read %s\n', NAME);
else
  nOscs = fscanf(fid, '%d', 1);
  % Increase the arrays in chunks of colchunk cols to avoid slow 
  % matrix growing.
  emptyF = zeros(nOscs, colchunk);
  F = emptyF;
  M = emptyF;
  Fcols = colchunk;
  
  endoffile = 0;
  while (endoffile == 0)
    [time,count] = fscanf(fid, '%f', 1);
    if (count == 0)
      endoffile = 1;
    else
      col = col+1;
      if(col > Fcols)
	% We ran out of empty columns - grow the matrices
	F = [F, emptyF];
	M = [M, emptyF];
	Fcols = Fcols + colchunk;
      end
      for osc = 1:nOscs
	F(osc,col) = fscanf(fid, '%f,', 1);
	M(osc,col) = fscanf(fid, '%f',1);
      end
    end
  end
  fclose(fid);
  % Trim off excess empty columns
  F = F(:,1:col);
  M = M(:,1:col);
end
