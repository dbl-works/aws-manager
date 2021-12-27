const { HttpRequest } = require('@aws-sdk/protocol-http')
const { SignatureV4 } = require('@aws-sdk/signature-v4')
const { Hash } = require('@aws-sdk/hash-node')
const { formatUrl } = require('@aws-sdk/util-format-url')

async function getAuthToken({ hostname, port, username, region, credentials }) {
  const protocol = 'https'

  const signer = new SignatureV4({
    service: 'rds-db',
    region,
    credentials,
    sha256: Hash.bind(null, 'sha256')
  })

  const request = new HttpRequest({
    method: 'GET',
    protocol,
    hostname,
    port,
    query: {
      Action: 'connect',
      DBUser: username
    },
    headers: {
      host: `${hostname}:${port}`,
    },
  })

  const presigned = await signer.presign(request, {
    expiresIn: 900
  })

  return formatUrl(presigned).replace(`${protocol}://`, '')
}

module.exports = getAuthToken