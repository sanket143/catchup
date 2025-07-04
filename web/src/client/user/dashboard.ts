import graphqlRequest from '@/client/graphqlRequest'

const query = `
  query getCurrentUser {
    user {
      id
      level
      username
      contests {
        id
        problems {
          id
          verdict
        }
      }
    }
  }
`

export default () => {
  return graphqlRequest({
    query,
  })
}
