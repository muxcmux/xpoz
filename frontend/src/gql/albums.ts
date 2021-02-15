export const getMyAlbums = `
  query($page: Int!) {
    myAlbums(page: $page) {
      id
      title
      photosCount
      videosCount
      createdAt
      keyAssets {
        id
      }
    }
  }
`;

export const getMyAlbumsForAccess = `
  query {
    myAlbums {
      id
      title
    }
  }
`;

export const getAlbum = `
  query($id: String!, $offset: Int!, $limit: Int!) {
    album(id: $id) {
      id
      title
      photosCount
      videosCount
      assets(offset: $offset, limit: $limit) {
        id
        width
        height
        isVideo
        duration
        createdAt
      }
    }
  }
`;
