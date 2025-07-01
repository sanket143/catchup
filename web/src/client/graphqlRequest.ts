import axios from 'axios'

export default ({ query, variables, headers = {} }) => {
  return axios({
    method: 'post',
    url: '/graphql',
    data: {
      query,
      variables,
    },
    headers,
  }).then(({ data: { data, errors } }) => {
    if (errors?.length > 0) {
      throw errors[0]
    }

    return data
  })
}
