import Link from "next/link";
import Image from "next/image";
import TwitterIcon from "../public/img/icon-twitter.svg";
import DiscordIcon from "../public/img/icon-discord.svg";
import YoutubeIcon from "../public/img/icon-youtube.svg";
import SteamIcon from "../public/img/icon-steam.svg";

const Footer = () => {
  return (
    <>
      <footer
        id="footer"
        className="px-[96px] pb-[50px] bg-black xs:pl-10 sm:pl-10"
      >
        <div className="flex flex-col items-center justify-between mx-auto md:flex-row md:items-start md:mt-0">
          <div id="logo" className="w-[70px]">
            <Link
              href="#"
              className="text-[40px] no-underline font-bold text-white font-[DotGothic16]"
            >
              Ataria
            </Link>
          </div>

          {/* Menus Container */}
          <div className="flex flex-col mt-14 md:flex-row md:mt-0 md:ml-20 xl:ml-80">
            {/* Menu 1 */}
            <div className="flex flex-col items-center w-full xs:mb-10 md:items-start md:mr-[50px] ml-[20px]">
              <div className="mb-[5px] font-bold text-[25px] text-white capitalize">
                Features
              </div>
              <div className="flex flex-col items-start mt-[12px]">
                <Link
                  href="#"
                  className="capitalize no-underline text-[#6c8df8] text-[20px] pb-[10px] hover:text-[#9274ff] duration-300"
                >
                  GStore
                </Link>
                <Link
                  href="#"
                  className="capitalize no-underline text-[#6c8df8] text-[20px] pb-[10px] hover:text-[#9274ff] duration-300"
                >
                  GLink
                </Link>
                <Link
                  href="#"
                  className="capitalize no-underline text-[#6c8df8] text-[20px] pb-[10px] hover:text-[#9274ff] duration-300"
                >
                  GThread
                </Link>
              </div>
            </div>
            {/* Menu 2 */}
            <div className="flex flex-col items-center w-full xs:mb-10 md:items-start md:mr-[50px] ml-[20px]">
              <div className="mb-[5px] font-bold text-[25px] text-white capitalize">
                Resources
              </div>
              <div className="flex flex-col items-start mt-[12px]">
                <Link
                  href="#"
                  className="capitalize no-underline text-[#6c8df8] text-[20px] pb-[10px] hover:text-[#9274ff] duration-300"
                >
                  Assets
                </Link>
                <Link
                  href="#"
                  className="capitalize no-underline text-[#6c8df8] text-[20px] pb-[10px] hover:text-[#9274ff] duration-300"
                >
                  Gallery
                </Link>
                <Link
                  href="#"
                  className="capitalize no-underline text-[#6c8df8] text-[20px] pb-[10px] hover:text-[#9274ff] duration-300"
                >
                  Support
                </Link>
              </div>
            </div>
            {/* Menu 3 */}
            <div className="flex flex-col items-center w-full md:items-start md:mr-[50px] ml-[20px]">
              <div className="mb-[5px] font-bold text-[25px] text-white capitalize">
                Services
              </div>
              <div className="flex flex-col items-start mt-[12px]">
                <Link
                  href="#"
                  className="capitalize no-underline text-[#6c8df8] text-[20px] pb-[10px] hover:text-[#9274ff] duration-300"
                >
                  About
                </Link>
                <Link
                  href="#"
                  className="capitalize no-underline text-[#6c8df8] text-[20px] pb-[10px] hover:text-[#9274ff] duration-300"
                >
                  FAQ
                </Link>
                <Link
                  href="#"
                  className="capitalize no-underline text-[#6c8df8] text-[20px] pb-[10px] hover:text-[#9274ff] duration-300"
                >
                  Contact
                </Link>
              </div>
            </div>
          </div>
          {/* Social Container */}
          <div className="flex space-x-6 xs:pl-10 xs:mt-10 sm:pl-10 sm:mt-10">
            <Link href="#">
              <Image
                src={TwitterIcon}
                alt="twitter"
                className="w-[40px] h-auto transition-[filter] duration-300 hover:filter hover:brightness-[89%] hover:contrast-[85%] hover:hue-rotate-[340deg] hover:invert-[76%] hover:saturate-[300%] hover:sepia-[61%]"
                id="twitter"
                width={10}
                height={10}
              />
            </Link>
            <Link href="#">
              <Image
                src={DiscordIcon}
                alt="discord"
                className="w-[40px] h-auto transition-[filter] duration-300 hover:filter hover:brightness-[89%] hover:contrast-[85%] hover:hue-rotate-[340deg] hover:invert-[76%] hover:saturate-[300%] hover:sepia-[61%]"
                id="discord"
                width={10}
                height={10}
              />
            </Link>
            <Link href="#">
              <Image
                src={YoutubeIcon}
                alt="youtube"
                className="w-[40px] h-auto transition-[filter] duration-300 hover:filter hover:brightness-[89%] hover:contrast-[85%] hover:hue-rotate-[340deg] hover:invert-[76%] hover:saturate-[300%] hover:sepia-[61%]"
                id="youtube"
                width={10}
                height={10}
              />
            </Link>
            <Link href="#">
              <Image
                src={SteamIcon}
                alt="steam"
                className="w-[40px] h-auto transition-[filter] duration-300 hover:filter hover:brightness-[89%] hover:contrast-[85%] hover:hue-rotate-[340deg] hover:invert-[76%] hover:saturate-[300%] hover:sepia-[61%]"
                id="steam"
                width={10}
                height={10}
              />
            </Link>
          </div>
        </div>
        {/* Copyright */}
        <div className="flex justify-center items-center pt-[60px] xs:pl-10 sm:pl-10">
          <h3 className="text-white">
            &copy;2024 <span className="copyright-title">Ataria</span>
          </h3>
        </div>
      </footer>
    </>
  );
};

export default Footer;
