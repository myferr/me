#!/usr/bin/env node

const { execa } = require("execa");
const path = require("path");

const COLOR_GREEN = "\x1b[32m";
const COLOR_YELLOW = "\x1b[33m";
const COLOR_RED = "\x1b[31m";
const COLOR_RESET = "\x1b[0m";

const projectRoot = __dirname;
const meDir = path.join(projectRoot, "me");

async function deploy() {
  console.log(`▲ DEPLOY.JS`);
    console.log(`${COLOR_YELLOW}Starting deployment process...${COLOR_RESET}`);

  try {
    console.log(`Vercel commands will be executed in: ${meDir}`);

    console.log(`${COLOR_YELLOW}Deploying to Vercel...${COLOR_RESET}`);
    await execa("vercel", ["deploy", "--prod"], {
      cwd: meDir,
      stdio: "inherit",
    });
    console.log(
      `${COLOR_GREEN}Deployment to Vercel completed successfully.${COLOR_RESET}`,
    );
  } catch (error) {
    console.error(`${COLOR_RED}Deployment failed:${COLOR_RESET}`, error);
    process.exit(1);
  }
}

deploy();
