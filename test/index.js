// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const { Config, Scenario } = require("@holochain/holochain-nodejs")
Scenario.setTape(require("tape"))

const dnaPath = "./dist/holochain_graphql_starter_kit.dna.json"
const agentAlice = Config.agent("alice")
const dna = Config.dna(dnaPath)
const instanceAlice = Config.instance(agentAlice, dna)
const scenario = new Scenario([instanceAlice])

scenario.runTape("description of example test", async (t, { alice }) => {

	// test making the most basic query
  const result = await alice.callSync("main", "graphql", {query : "{ apiVersion }", variables: {}})
  t.deepEqual(result, { Ok: `{"apiVersion":"1.0"}`})

})
