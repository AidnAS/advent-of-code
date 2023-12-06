s,a=lambda t,r:int((t**2-4*r)**.5)+1,1
t=zip(*[map(int,[''.join(k)]+k)for _,*k in map(str.split,open(0))]);b=s(*next(t))
for p in t:a*=s(*p)
print(a,b)