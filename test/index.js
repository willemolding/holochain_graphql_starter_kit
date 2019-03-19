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
  // define some helpers
	let results = []
	const lastResult = (back=0) => results[results.length-1-back]
	const queryHolochain = async (query, variables) => {
		const result = await alice.callSync("main", "graphql", {query, variables})
		results.push(result)
		return result
	}

	// test making the most basic query
  await queryHolochain("{ apiVersion }", {})
  t.deepEqual(JSON.parse(lastResult().Ok), { apiVersion: "1.0" })

  // get the root 
	await queryHolochain(`
	{ 
		rootWidget { 
			address,
			description 
		} 
	}`, {})
  t.deepEqual(JSON.parse(lastResult().Ok).rootWidget.description, "root")
  const rootWidgetAddress = JSON.parse(lastResult().Ok).rootWidget.address
	t.equal(rootWidgetAddress.length, 46)


  // Add a new widget and make it a child of the root
  // This also demonstrates how to use parameters/variables in queries
 	await queryHolochain(`
  	mutation AddAWidget($description: String!) {
  		addWidget(description: $description) { 
  			address 
  		}
  	}
  	`, {description: "child 1"})
 	
 	const newWidgetAddress = JSON.parse(lastResult().Ok).addWidget.address
 	t.equal(newWidgetAddress.length, 46)

 	// note juniper automatically converts parameters to camelCase from lower_snake_case
 	await queryHolochain(`
  	mutation AddChildWidget($parent: ID!, $child: ID!) {
  		appendSubwidget(parentAddress: $parent, childAddress: $child) { address }
  	}
  	`, {parent: rootWidgetAddress, child: newWidgetAddress})


 	// make a structured query to return the widget tree
 	// only look three layers deep for now
 	await queryHolochain(`{ 
 		rootWidget { 
 			description, 
 			subwidgets {
 				description,
 				subwidgets { 
 					description 
 				}
 			} 
 		} 
 	}`, {})

 	t.deepEqual(JSON.parse(lastResult().Ok), {
 		rootWidget:{
 			description:"root",
 			subwidgets:[
 				{
 					description:"child 1",
 					subwidgets:[]
 				}
 			]
 		}
 	})

 	// print in order all the outputs from the queries
  results.forEach((r, i) => {
  	console.log(i, r)
  })

})
