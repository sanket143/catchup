import graphqlRequest from '@/client/graphqlRequest'

const query = `
  query ProblemTagGroups {
    problemTagGroups {
      id
      name
      contests {
        id
        name
        duration
        startedOn
        createdFor
        problems {
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
