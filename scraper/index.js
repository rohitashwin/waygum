const puppeteer = require('puppeteer');
const { Mutex, Semaphore, withTimeout } = require('async-mutex');

let youtubeTitles = [];
const storeMutex = new Mutex();

async function storeTitleToList(title) {
	const release = await storeMutex.acquire();
	try {
		youtubeTitles.push(title);
	} finally {
		release();
	}
}

async function getHandlerInnerHTML(handler) {
	const innerHTMLProperty = await handler.getProperty('innerHTML');
	const stringValue = await innerHTMLProperty.jsonValue();
	return stringValue;
}

async function get_youtube_title(youtube_page) {
	// ytd-compact-video-renderer chosen arbitrarily since it was one of the last items to load :D
	await youtube_page.waitForSelector('ytd-compact-video-renderer');
	await youtube_page.screenshot({path: 'screenshot.png'});
	const title = await youtube_page.$('#title.ytd-watch-metadata');
	const innerTitles = await title.$('h1 > yt-formatted-string');
	console.log(await getHandlerInnerHTML(innerTitles));
}

async function scraper() {
	const browser = await puppeteer.launch();
	const userAgent = 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:107.0) Gecko/20100101 Firefox/107.0';
	const page = await browser.newPage();
	await page.goto('https://www.youtube.com/watch?v=yV52TMdGkng');
	await page.setUserAgent(userAgent);
	await get_youtube_title(page);
}

scraper(); 