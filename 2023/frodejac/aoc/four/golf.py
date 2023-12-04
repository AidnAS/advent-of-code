d=[*open(0)];a,b,c=0,0,[1]*len(d)
for l in d:w,h=map(set,map(str.split,l.split('|')));n=len(w&h);a+=1<<n>>1;i,*c=c;b+=i;j=0;exec('c[j]+=i;j+=1;'*n)
print(a,b)