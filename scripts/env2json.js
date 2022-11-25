const path = require("path");
const dotenv = require("dotenv");
const dotenvExpand = require("dotenv-expand");
const fs = require("fs");

const DOTENV = /\.env$/;

(function main() {
    const dir = path.resolve(__dirname, "../tests/fixtures");
    Promise.all(dotenvs(dir).map(convert)).then((files) => {
        console.debug(`done with ${files.length} files!\n`);
    });
})();

function dotenvs(dir) {
    return fs
        .readdirSync(dir)
        .filter((it) => DOTENV.test(it))
        .map((it) => path.resolve(dir, it));
}

/**
 * @param {string} filePath
 * @returns {Promise<string>}
 */
function convert(filePath) {
    const env = dotenv.config({
        path: filePath,
    });

    const out = dotenvExpand.expand(env);
    return new Promise((resolve, reject) => {
        const outPath = filePath.replace(DOTENV, ".json");
        const outData = JSON.stringify(out.parsed, null, 4) + "\n";
        console.debug(`- convert ${outPath} ...`);
        fs.writeFile(outPath, outData, (err) => {
            err ? reject(err) : resolve(outPath);
        });
    });
}
