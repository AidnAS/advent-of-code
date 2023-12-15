s,a=lambda t,r:int((t*t-4*r)**.5)+1,1
b,*t=zip(*[map(int,[''.join(k)]+k)for _,*k in map(str.split,open(0))]);b=s(*b)
for p in t:a*=s(*p)
print(a,b)