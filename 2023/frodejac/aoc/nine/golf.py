a=b=0;m=lambda s,f:any(s)and s[-f]+(f*2-1)*m([j-i for i,j in zip(s,s[1:])],f)or 0
for*s,in[map(int,l.split())for l in open(0)]:a+=m(s,1);b+=m(s,0)
print(a,b)