import {sum} from 'lodash';

const isMine = (c: string) => c === '*';

const mineToStr = (m: number) => (m === 0 ? '.' : m === -1 ? '*' : String(m));

interface Adjacent {
  i: number;
  j: number;
}

const checkMinesAdjacents = (
  rows: number,
  cols: number,
  i: number,
  j: number
): Adjacent[] =>
  [
    {i: i - 1, j: j - 1},
    {i, j: j - 1},
    {i: i + 1, j: j - 1},
    {i: i - 1, j},
    {i: i + 1, j},
    {i: i - 1, j: j + 1},
    {i, j: j + 1},
    {i: i + 1, j: j + 1},
  ].filter(a => a.i >= 0 && a.j >= 0 && a.i < cols && a.j < rows);

const genMineSweeper = (mine2d: string[]): number[][] => {
  const rows = mine2d.length;
  const cols = mine2d[0].length;
  return mine2d.map((rowStr, j) =>
    rowStr.split('').map((c, i) =>
      isMine(c)
        ? -1
        : sum(
            checkMinesAdjacents(rows, cols, i, j) //
              .map(ad => isMine(mine2d[ad.j][ad.i]))
          )
    )
  );
};

const main = () => {
  const sampleMineSweeper = [
    '·*·*·', //
    '··*··',
    '··*··',
    '·····',
  ];

  const lines = genMineSweeper(sampleMineSweeper) //
    .map(ys => ys.map(mineToStr).join(''));

  lines.forEach(l => console.log(l));
};

main();
