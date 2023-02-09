const axios = require('axios');
const Octokit = require('@octokit/rest')
const octokit = new Octokit({
    auth: process.env.secrets_TOKEN
})
axios.get("https://api.github.com/repos/weykon/bit-playground/issues?per_page=3").then((res) => {
    console.log(res)
})
 