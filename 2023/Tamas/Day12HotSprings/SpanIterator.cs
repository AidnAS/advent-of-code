using System;
using System.Reflection.Metadata.Ecma335;

namespace Day12HotSprings;

internal readonly ref struct SpanIterator<T>
{
    private readonly ReadOnlySpan<T> span;
    private readonly int subtractFrom;
    private readonly int multiplyBy;

    public SpanIterator(
        ReadOnlySpan<T> span,
        bool backward)
    {
        this.span = span;
        (subtractFrom, multiplyBy) = backward
            ? (span.Length - 1, 1)
            : (0, -1);
    }

    public static implicit operator SpanIterator<T>(
        ReadOnlySpan<T> span)
    {
        return SpanIterator.For(span);
    }

    public int Length => span.Length;
    public T this[int i] => span[unchecked((subtractFrom - i) * multiplyBy)];
    public bool IsBackward => multiplyBy == 1;

    public SpanIterator<T> GetReversed()
    {
        return new SpanIterator<T>(
            span, !IsBackward);
    }

    public SpanIterator<T> Slice(
        int start)
    {
        return this[start..Length];
    }

    public SpanIterator<T> Slice(
        int start,
        int length)
    {
        var spanSlice = IsBackward
            ? span.Slice(span.Length - start - length, length)
            : span.Slice(start, length);
        return new SpanIterator<T>(
            spanSlice,
            IsBackward);
    }
}

internal static class SpanIterator
{
    public static SpanIterator<T> For<T>(
        ReadOnlySpan<T> span)
    {
        return new SpanIterator<T>(span, false);
    }

    public static SpanIterator<T> BackwardFor<T>(
        ReadOnlySpan<T> span)
    {
        return new SpanIterator<T>(span, true);
    }

    public static SpanIterator<T> Empty<T>()
    {
        return new SpanIterator<T>(
            ReadOnlySpan<T>.Empty, false);
    }
}
