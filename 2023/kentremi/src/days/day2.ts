import { FileHandle } from 'fs/promises';

async function day2(task: number, dayFileHandle: FileHandle) {
  let result = 0;
  for await (const line of dayFileHandle.readLines()) {
    const gameIdMatch = /Game (\d+)/.exec(line);
    const gamesMatch = /Game \d+:\s(.+)/.exec(line);

    if (!gameIdMatch || !gamesMatch) return 'DATA ERROR!';

    const gameId = gameIdMatch[1];
    const gamesData = gamesMatch[1].split(';');
    let invalid = false;

    gamesData.forEach((gameData) => {
      gameData.split(',').every((game) => {
        const cubes = /(\d+)\s(red|blue|green)/.exec(game.trim());
        if (!cubes) return;

        switch (cubes[2]) {
          case 'red':
            if (parseInt(cubes[1]) > 12) {
              invalid = true;
            }
            break;
          case 'green':
            if (parseInt(cubes[1]) > 13) {
              invalid = true;
            }
            break;
          case 'blue':
            if (parseInt(cubes[1]) > 14) {
              invalid = true;
            }
            break;
        }

        if (invalid) return false;
        return true;
      });
    });

    if (!invalid) {
      result += parseInt(gameId);
    }
  }

  return result.toString();
}

export default day2;
