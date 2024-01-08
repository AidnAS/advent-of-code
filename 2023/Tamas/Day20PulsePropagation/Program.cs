using Day20PulsePropagation;

var mainBlueprint = new Blueprint(
    File.ReadAllText("input.txt"),
    new string[] { "broadcaster" },
    Array.Empty<string>());
var bus = new Bus();
var button = new Button(bus, "button", false);
var mainCircuit = mainBlueprint.Build(bus);
mainCircuit.ConnectAllInputsTo(button);

var counterMultiplier = SignalCounterMultiplierBuilder.Default.Build(bus, "--counter-");
counterMultiplier.ConnectAllInputsTo(button);
foreach (var module in mainCircuit.ModuleMap.Values)
{
    int connectedModuleCount = module.DownstreamPins.Count;
    for (int i = 0; i < connectedModuleCount; i++)
    {
        counterMultiplier.ConnectAllInputsTo(module);
    }
}

var accumulator = Int64Reader.Create(counterMultiplier, "acc-");

for (int i = 0; i < 1000; i++)
{
    button.Push();
    bus.Run();
}

Console.WriteLine($"The product after the warm up is {accumulator.Read()}.");

button = new Button(bus, "button", false);
mainCircuit = mainBlueprint.Build(bus);
mainCircuit.ConnectAllInputsTo(button);
var rx = mainCircuit["rx"];
var finishedTokenSource = new CancellationTokenSource();
rx.OnSignalReceived = signal =>
{
    if (!signal)
    {
        finishedTokenSource.Cancel();
    }
};

ulong buttonPressCount = 0;
var finishedToken = finishedTokenSource.Token;
while (!finishedToken.IsCancellationRequested)
{
    buttonPressCount++;
    button.Push();
    bus.Run();
}

// ...after a little more than a year...

Console.WriteLine($"Rx gets the first low pulse after {buttonPressCount} button presses.");

// Since I don't want to wait that long, I just solved part 2 on paper. Silly task.

