export const getTokens = `
  query {
    tokens {
      __typename
      id
      name
      sessionId
      sessionBound
      admin
      createdAt
      whitelistedAlbums {
        id
      }
    }
  }
`;

export const me = `
  query {
    me {
      id
      name
      admin
    }
  }
`

export const createToken = `
  mutation($input: TokenInput!) {
    createToken(input: $input) {
      __typename
      id
      name
      sessionId
      sessionBound
      admin
      createdAt
      whitelistedAlbums {
        id
      }
    }
  }
`;

export const updateToken = `
  mutation($id: String!, $input: TokenInput!) {
    updateToken(id: $id, input: $input) {
      __typename
      id
    }
  }
`;

export const removeToken = `
  mutation($id: String!) {
    deleteToken(id: $id) {
      __typename
      id
    }
  }
`

