#!/bin/env bun

import { $ } from "bun";
import chalk from "chalk";

$.env({
  ...process.env,
  FORCE_COLOR: "1",
});

async function buildClient() {
  console.log(chalk.blue("--> Building `client`..."));
  await $`yarn build`.cwd("client");
  console.log(chalk.green("--> Built `client`!"));
}

async function buildServer() {
  console.log(chalk.blue("--> Building `server`..."));
  await $`cargo build --release --features bundle-client`.cwd("server");
  console.log(chalk.green("--> Built `server`!"));
}

await buildClient();
await buildServer();
