import http from 'http'

const server = http.createServer((req, res) => {
  let body = []

  req.on('data', chunk => body.push(chunk))

  req.on('end', () => {
    const totalBytes = body.reduce((sum, chunk) => sum + chunk.length, 0)

    console.log('─────────────────────────────')
    console.log(`File     : ${req.headers['x-file-name']}`)
    console.log(`Upload ID: ${req.headers['x-upload-id']}`)
    console.log(`Chunk    : ${req.headers['x-chunk-index']} / ${req.headers['x-total-chunks']}`)
    console.log(`Bytes    : ${totalBytes}`)
    console.log(`Time     : ${new Date().toLocaleTimeString()}`)

    res.writeHead(200, { 'Content-Type': 'application/json' })
    res.end(JSON.stringify({ success: true }))
  })
})

server.listen(3000, '127.0.0.1', () => {
  console.log('✅ Test upload server running at http://localhost:3000')
  console.log('Waiting for chunks...\n')
})