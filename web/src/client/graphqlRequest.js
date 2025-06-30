import axios from 'axios'

export default ({ query, mutation, variables, headers = {} }) => {
  return axios({
    method: 'post',
    url: '/graphql',
    data: {
      query,
      mutation,
      variables,
    },
    headers,
  })
}
