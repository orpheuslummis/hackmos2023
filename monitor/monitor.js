const fetch = require('node-fetch');
const url = 'http://localhost:26657/status';

async function fetchData() {
  console.log("=======================================================");
  try {
    const response = await fetch(url);
    
    if (!response.ok) {
      throw new Error('Network response was not ok');
    }

    const data = await response.json();
    const height = data.result["sync_info"]["latest_block_height"];
    const url2 = "http://localhost:26657/block_results?height="+height;
    const response2 = await fetch(url2);
    const data2 = await response2.json();
    const result = data2.result;
    console.log(result);
  } catch (error) {
    console.error('There was a problem:', error.message);
  }
}

setInterval(fetchData, 10000);





