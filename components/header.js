import Image from 'next/image';
import styles from '../styles/Home.module.css';

export default function Header() {
  return (
    <div className={styles.header_grid}>
      <p className={styles.header}>Just GIFS</p>
      <Image
        className={styles.gif}
        height="70"
        width="70"
        alt="rick"
        src="/tenor.gif"
      />
    </div>
  );
}
