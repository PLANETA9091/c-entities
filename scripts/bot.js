// Fake player for baseline runs: joins the offline-mode Paper server and
// stays AFK at spawn so Paper's Entity Activation Range sees a player.
const mineflayer = require('mineflayer')

const host = process.env.BOT_HOST || '127.0.0.1'
const port = parseInt(process.env.BOT_PORT || '25565', 10)

function connect(attempt) {
  const bot = mineflayer.createBot({
    host,
    port,
    username: 'baseline_bot',
    auth: 'offline',
    version: '1.21.8',
  })
  bot.on('login', () => console.log('BOT-LOGGED-IN'))
  bot.on('spawn', () => console.log('BOT-SPAWNED'))
  bot.on('kicked', (r) => console.log('BOT-KICKED', String(r).slice(0, 200)))
  bot.on('error', (e) => console.log('BOT-ERROR', e && e.message))
  bot.on('end', (r) => {
    console.log('BOT-END', String(r).slice(0, 200))
    if (attempt < 30) setTimeout(() => connect(attempt + 1), 3000)
  })
}

connect(0)
