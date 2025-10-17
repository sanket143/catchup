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
    url: 'https://gql-catchup.sanket143.me/',
    data: {
      query,
      variables,
    },
    headers,
    withCredentials: true,
  }).then(({ data: { data, errors } }) => {
    if (errors?.length > 0) {
      throw errors[0]
    }

    return data
  })
}
