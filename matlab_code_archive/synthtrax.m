function X = synthtrax(F, M, SR, SUBF, DUR)
% X = synthtrax(F, M, SR, SUBF, DUR)      Reconstruct a sound from track rep'n.
%	Each row of F and M contains a series of frequency and magnitude 
%	samples for a particular track.  These will be remodulated and 
%	overlaid into the output sound X which will run at sample rate SR, 
%	although the columns in F and M are subsampled from that rate by 
%	a factor SUBF (default 128).  If DUR is nonzero, X will be padded or
%	truncated to correspond to just this much time.
% dpwe@icsi.berkeley.edu 1994aug20, 1996aug22

if(nargin<4)
  SUBF = 128;
end

if(nargin<5)
  DUR = 0;
end

rows = size(F,1);
cols = size(F,2);

opsamps = round(DUR*SR);
if(DUR == 0)
  opsamps = 1 + ((cols-1)*SUBF);
end

X = zeros(1, opsamps);

for row = 1:rows
%  fprintf(1, 'row %d.. \n', row);
  mm = M(row,:);
  ff = F(row,:);
  % Where mm = 0, ff is undefined.  But interp will care, so find points 
  % and set.
  % First, find onsets - points where mm goes from zero (or NaN) to nzero
  % Before that, even, set all nan values of mm to zero
  mm(find(isnan(mm))) = zeros(1, sum(isnan(mm)));
  ff(find(isnan(ff))) = zeros(1, sum(isnan(ff)));
  nzv = find(mm);
  firstcol = min(nzv);
  lastcol = max(nzv);
  % for speed, chop off regions of initial and final zero magnitude - 
  % but want to include one zero from each end if they are there 
  zz = [max(1, firstcol-1):min(cols,lastcol+1)];
  mm = mm(zz);
  ff = ff(zz);
  nzcols = prod(size(zz));
  mz = (mm==0);
  mask = mz & (0==[mz(2:nzcols),1]);
  ff = ff.*(1-mask) + mask.*[ff(2:nzcols),0];
  % Do offsets too
  mask = mz & (0==[1,mz(1:(nzcols-1))]);
  ff = ff.*(1-mask) + mask.*[0,ff(1:(nzcols-1))];
  % Ok. Can interpolate now
  % This is actually the slow part
%  % these parameters to interp make it do linear interpolation
%  ff = interp(ff, SUBF, 1, 0.001);
%  mm = interp(mm, SUBF, 1, 0.001);
%  % chop off past-the-end vals from interp
%  ff = ff(1:((nzcols-1)*SUBF)+1);
%  mm = mm(1:((nzcols-1)*SUBF)+1);
  % slinterp does linear interpolation, doesn't extrapolate, 4x faster
  ff = slinterp(ff, SUBF);
  mm = slinterp(mm, SUBF);
  % convert frequency to phase values
  pp = cumsum(2*pi*ff/SR);
  % run the oscillator and apply the magnitude envelope
  xx = mm.*cos(pp);
  % add it in to the correct place in the array
  base = 1+SUBF*(zz(1)-1);
  sizex = prod(size(xx));
  ww = (base-1)+[1:sizex];
  X(ww) = X(ww) + xx;
end
