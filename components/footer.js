import Image from 'next/image';
import styles from '../styles/Home.module.css';

// Constants
const TWITTER_HANDLE = 'tapabratadey';
const TWITTER_LINK = `https://twitter.com/${TWITTER_HANDLE}`;
const GITHUB_HANDLE = 'tapabratadey';
const GITHUB_LINK = `https://twitter.com/${GITHUB_HANDLE}`;

export default function Header() {
  return (
    <div className={styles.footerContainer}>
      <a className={styles.footerText}>{`built by @${TWITTER_HANDLE}`}</a>
      <a
        href={TWITTER_LINK}
        target="_blank"
        alt="Twitter Logo"
        rel="noreferrer"
      >
        <Image
          height="20"
          width="20"
          alt="next"
          src="/twitter.svg"
          className={styles.twitter}
        />
      </a>
      <a href={GITHUB_LINK} target="_blank" alt="Github Logo" rel="noreferrer">
        <Image
          height="20"
          width="20"
          alt="next"
          src="/github.svg"
          className={styles.github}
        />
      </a>
    </div>
  );
}
