#!/usr/bin/env -S deno run --allow-read

// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

import * as io from "@std/io";

import * as command from "@cliffy/command";

import * as ron from "../pkg/ron_wasm.js";

import { VERSION } from "./version.ts";

const { args } = await new command.Command()
  .name("json2ron")
  .version(VERSION)
  .description("An example of converting a JSON string to a RON string.")
  .arguments("[FILE:file]")
  .parse();

const input = args[0] === undefined
  ? io.readAllSync(Deno.stdin)
  : Deno.readFileSync(args[0]);
const string = new TextDecoder().decode(input);
const obj = JSON.parse(string);

const output = ron.stringify(obj);
console.log(output);
