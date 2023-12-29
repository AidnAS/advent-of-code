namespace Day15LensLibrary;

internal sealed class Facility
{
    private const char InsertOperation = '=';
    private const char RemoveOperation = '-';
    private static readonly char[] AllOperations = new char[] { InsertOperation, RemoveOperation };

    private readonly Box[] boxes;

    public Facility()
    {
        boxes = new Box[256];
        for (int boxNumber = 0; boxNumber < boxes.Length; boxNumber++)
        {
            boxes[boxNumber] = new Box(boxNumber);
        }
    }

    public void Execute(
        IReadOnlyList<string> instructions)
    {
        foreach (var instruction in instructions)
        {
            int operationIndex = instruction.IndexOfAny(AllOperations);
            var operation = instruction[operationIndex];
            string label = instruction[0..operationIndex];
            int boxNumber = HashAlgorithm.Run(label);
            var box = boxes[boxNumber];

            var oldLensSlot = box.Lenses.First;
            while (oldLensSlot != null)
            {
                if (oldLensSlot.Value.Label == label)
                {
                    break;
                }
                oldLensSlot = oldLensSlot.Next;
            }

            if (operation == InsertOperation)
            {
                int focalLength = int.Parse(instruction[(operationIndex + 1)..]);
                var newLens = new Lens(focalLength, label);
                if (oldLensSlot != null)
                {
                    oldLensSlot.Value = newLens;
                }
                else
                {
                    box.Lenses.AddLast(newLens);
                }
            }
            else if (operation == RemoveOperation)
            {
                if (oldLensSlot != null)
                {
                    box.Lenses.Remove(oldLensSlot);
                }
            }    
        }
    }

    public int GetCombinedFocusingPower()
    {
        int sum = 0;
        foreach (var box in boxes)
        {
            int lensIndex = 0;
            foreach (var lens in box.Lenses)
            {
                sum += (box.BoxNumber + 1) 
                    * (lensIndex + 1) 
                    * lens.FocalLength;
                lensIndex++;
            }
        }
        return sum;
    }
}
