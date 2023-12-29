using System.Collections;

namespace Day14ParabolicReflectorDish;

internal class MultiMapLookup<TKey, TElement> : ILookup<TKey, TElement> 
    where TKey : notnull
{
    private readonly List<Dictionary<TKey, TElement>> mapList;
    private readonly Dictionary<TKey, TElement> firstMap;

    public MultiMapLookup()
    {
        firstMap = new Dictionary<TKey, TElement>();
        mapList = new List<Dictionary<TKey, TElement>>() { firstMap };
    }

    public IEnumerable<TElement> this[TKey key]
    {
        get
        {
            return firstMap.TryGetValue(key, out var element)
                ? Lookup(key, element)
                : Enumerable.Empty<TElement>();
        }
    }

    public int Count => firstMap.Count;

    public bool Contains(TKey key)
    {
        return firstMap.ContainsKey(key);
    }

    public void Add(
        TKey key,
        TElement element)
    {
        foreach (var map in mapList)
        {
            if (map.TryAdd(key, element))
            {
                return;
            }
        }
        var newMap = new Dictionary<TKey, TElement>();
        mapList.Add(newMap);
        newMap.Add(key, element);
    }

    public void Clear()
    {
        firstMap.Clear();
        mapList.Clear();
        mapList.Add(firstMap);
    }

    public IEnumerator<IGrouping<TKey, TElement>> GetEnumerator()
    {
        throw new NotImplementedException();
    }

    IEnumerator IEnumerable.GetEnumerator()
    {
        throw new NotImplementedException();
    }

    private IEnumerable<TElement> Lookup(
        TKey key,
        TElement firstElement)
    {
        yield return firstElement;
        for (int i = 1; i < mapList.Count; i++)
        {
            var map = mapList[i];
            if (!map.TryGetValue(key, out var element))
            {
                yield break;
            }

            yield return element;
        }
    }
}
