import axios from 'axios'

export default async ({
  query,
  variables = {},
  headers = {},
}: {
  query: string
  variables?: object
  headers?: object
}) => {
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
