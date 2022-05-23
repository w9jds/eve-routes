const path = require('path');
const fs = require('fs/promises');

const filePath = path.resolve(__dirname, './jump_map.json');

const main = async () => {

  const data = await fs.readFile(filePath);

  const info = JSON.parse(data);

  const map = info.reduce((out, current) => {
    out[current.id] = {
      name: current.name,
      x: current.x,
      y: current.y,
      z: current.z,
      security: current.security,
      neighbors: current.neighbors,
    };

    return out;
  }, {});

  fs.writeFile('../system_map.json', JSON.stringify(map));
}

main();