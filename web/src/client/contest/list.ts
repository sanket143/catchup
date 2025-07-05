import graphqlRequest from '@/client/graphqlRequest'

const query = `
  query getCurrentUser {
    user {
      id
      username
      recentContest {
        id
        name
        duration
        startedOn
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
      contests {
        id
        name
        problems {
          id
          isEvaluated
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
