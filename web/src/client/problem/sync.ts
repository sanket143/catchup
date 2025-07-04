import graphqlRequest from '@/client/graphqlRequest'

const query = `
  mutation SyncProblemList {
    syncProblemList
  }
`

export default () => {
  return graphqlRequest({
    query,
  })
}
