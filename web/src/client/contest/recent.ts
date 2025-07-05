import graphqlRequest from '@/client/graphqlRequest'

const query = `
  query getCurrentUser {
    user {
      id
      username
      recentContest {
        id
        name
        isEvaluated
        problems {
          id
          latestSubmissionAt
          isEvaluated
          verdict
          problem {
            id
            uid
            url
            title
            rating
          }
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
