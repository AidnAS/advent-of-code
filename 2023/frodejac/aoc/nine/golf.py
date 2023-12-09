a=b=0
for*s,in[map(int,l.split())for l in open(0)]:
 k=[s];g=h=0
 while any(k[0]):k=[[j-i for i,j in zip(k[0],k[0][1:])]]+k
 for p in k:g+=p[-1];h=p[0]-h
 a+=g;b+=h
print(a,b)
