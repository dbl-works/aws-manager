# RDS Manager
Provides a simple way to get password to AWS RDS instances in form of Electron app.

# How to install

```sh
-> yarn install
-> yarn start
```

# How to build

## With an ARM chip ("M1 Mac")

### Build for Mac

```shell
yarn make -p darwin -a x64
yarn make -p darwin -a arm64
```

### Build for Linux

You might need to install some extra packages:
```shell
brew install dpkg
brew install fakeroot
brew install rpm
```

then
```shell
yarn make -p linux -a x64
yarn make -p linux -a arm64
```

# How to contribute
* Open an issue to start a conversation
* Open a pull request
